mod command_item;
mod commands;
mod config;
mod file_service;
mod utils;
use command_item::CommandItem;
use commands::Commands;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Command,
};
use itertools::Itertools;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Tabs, Wrap},
    Frame, Terminal,
};

struct StatefulList {
    command_state: ListState,
    namespace_state: ListState,
    items: Vec<CommandItem>,
    namespaces: Vec<String>,
    current_namespace: String,
}

impl StatefulList {
    fn with_items(items: Vec<CommandItem>, namespaces: Vec<String>) -> StatefulList {
        StatefulList {
            command_state: ListState::default(),
            namespace_state: ListState::default(),
            items,
            namespaces,
            current_namespace: String::from("All"),
        }
    }

    fn next(&mut self) {
        let i = match self.command_state.selected() {
            Some(i) => {
                if i >= self.filtered_commands().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.command_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.command_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_commands().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.command_state.select(Some(i));
    }

    fn next_namespace(&mut self) {
        let i = match self.namespace_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.namespace_state.select(Some(i));
        self.current_namespace = String::from(self.namespaces.get(i).unwrap_or(&"All".to_string()));
        self.command_state.select(Some(0));
    }

    fn previous_namespace(&mut self) {
        let i = match self.namespace_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.namespace_state.select(Some(i));
        self.current_namespace = String::from(self.namespaces.get(i).unwrap_or(&"All".to_string()));
        self.command_state.select(Some(0));
    }

    fn filtered_commands(&self) -> Vec<CommandItem> {
        self.items
            .clone()
            .into_iter()
            .filter(|c| {
                if self.current_namespace == "All" {
                    true
                } else {
                    c.namespace == self.current_namespace
                }
            })
            .collect::<Vec<CommandItem>>()
    }
}

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
struct App {
    items: StatefulList,
}

impl App {
    fn new(command_items: Vec<CommandItem>, namespaces: Vec<String>) -> App {
        App {
            items: StatefulList::with_items(command_items, namespaces),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal

    file_service::load_commands_file();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut command_app = commands::Commands::init();
    command_app.namespaces().insert(0, String::from("All"));
    // create app and run it
    let mut app = App::new(command_app.clone().items, command_app.clone().namespaces());
    app.items.command_state.select(Some(0));
    app.items.namespace_state.select(Some(0));
    app.items.namespaces.insert(0, String::from("All"));
    let res = run_app(&mut terminal, app, command_app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    commands: Commands,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app, commands.clone()))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Left => app.items.previous_namespace(),
                KeyCode::Down => app.items.next(),
                KeyCode::Up => app.items.previous(),
                KeyCode::Right => app.items.next_namespace(),
                _ => {}
            }
        }
    }

    fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, commands: Commands) {
        // Create two chunks with equal horizontal screen space
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(f.size());

        //create tab menu items
        let mut tab_menu: Vec<Spans> = commands
            .clone()
            .namespaces()
            .into_iter()
            .map(|t| Spans::from(vec![Span::styled(t.clone(), Style::default())]))
            .collect();

        //includes All item (should occur before this method)
        tab_menu.insert(
            0,
            Spans::from(vec![Span::styled(String::from("All"), Style::default())]),
        );

        //create tab menu widget
        let tabs = Tabs::new(tab_menu)
            .select(app.items.namespace_state.selected().unwrap())
            .block(Block::default().title(" Namespaces ").borders(Borders::ALL))
            .style(Style::default())
            .highlight_style(
                Style::default()
                    .fg(Color::Rgb(181, 118, 20))
                    .add_modifier(Modifier::UNDERLINED),
            )
            .divider(Span::raw("|"));

        // Create the command items list
        let list_items: Vec<ListItem> = app
            .items
            .filtered_commands()
            .into_iter()
            .map(|c| {
                let lines = vec![Spans::from(c.alias.clone().unwrap_or(String::from("-")))];

                ListItem::new(lines.clone().to_owned())
                    .style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        // Create the command items widget
        let items = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title(" Commands "))
            .highlight_style(
                Style::default()
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        let idx = app
            .items
            .command_state
            .selected()
            .expect("need to select a current command");

        let selected_command: CommandItem = app.items.filtered_commands().get(idx).unwrap().clone();
        let command_str: String = selected_command.command;

        //Show the current command
        let command = Paragraph::new(command_str)
            .style(Style::default())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default())
                    .title(" Selected command ")
                    .border_type(BorderType::Plain),
            );

        let commands_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(chunks[1]);
        let command_detail_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Percentage(60),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(commands_chunks[1]);

        f.render_widget(command, command_detail_chunks[0]);
        f.render_widget(tabs, chunks[0]);
        f.render_stateful_widget(items, commands_chunks[0], &mut app.items.command_state);
    }
}

// fn render_commands<'a>(
//     commands_list: Vec<CommandItem>,
//     app: &mut App,
// ) -> (
//     List<'a>,
//     Paragraph<'a>,
//     Paragraph<'a>,
//     Paragraph<'a>,
//     Paragraph<'a>,
// ) {
//     let commands = Block::default()
//         .borders(Borders::ALL)
//         .style(Style::default())
//         .title(" Commands ")
//         .border_type(BorderType::Plain);

//     let items: Vec<_> = commands_list
//         .iter()
//         .map(|command| {
//             ListItem::new(Spans::from(vec![Span::styled(
//                 command.alias.as_ref().unwrap().clone(),
//                 Style::default(),
//             )]))
//         })
//         .collect();
//     let idx = app
//         .items
//         .state
//         .selected()
//         .expect("there is always a selected command");
//     let selected_command: &CommandItem = commands_list.get(idx).unwrap();

//     if selected_command.alias.as_ref().is_none() {
//         // If somehow the selection is past the last index, set it to the last element
//         let new_selection = if commands_list.is_empty() {
//             0
//         } else {
//             commands_list.len() - 1
//         };
//         app.items.state.select(Some(new_selection));
//     }

//     let list = List::new(items)
//         .block(commands)
//         .highlight_style(Style::default().add_modifier(Modifier::BOLD));

//     let command = Paragraph::new(selected_command.command.clone())
//         .style(Style::default())
//         .alignment(Alignment::Left)
//         .wrap(Wrap { trim: true })
//         .block(
//             Block::default()
//                 .borders(Borders::ALL)
//                 .style(Style::default())
//                 .title(" Hoarded command ")
//                 .border_type(BorderType::Plain),
//         );

//     let tags = Paragraph::new(
//         selected_command
//             .tags
//             .as_ref()
//             .unwrap_or(&vec![String::from("")])
//             .join(","),
//     )
//     .style(Style::default())
//     .alignment(Alignment::Left)
//     .block(
//         Block::default()
//             .borders(Borders::ALL)
//             .style(Style::default())
//             .title(" Tags ")
//             .border_type(BorderType::Plain),
//     );

//     let description = Paragraph::new(
//         selected_command
//             .description
//             .as_ref()
//             .unwrap_or(&"".to_string())
//             .to_string(),
//     )
//     .style(Style::default())
//     .alignment(Alignment::Left)
//     .wrap(Wrap { trim: true })
//     .block(
//         Block::default()
//             .borders(Borders::ALL)
//             .style(Style::default())
//             .title(" Description ")
//             .border_type(BorderType::Plain),
//     );

//     let query_title = format!(" cl v0.1.0 ");
//     let input = Paragraph::new("version").block(
//         Block::default()
//             .style(Style::default())
//             .borders(Borders::ALL)
//             .title(query_title),
//     );

//     (list, command, tags, description, input)
// }
