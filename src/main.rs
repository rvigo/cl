mod command_item;
mod commands;
mod config;
mod file_service;
mod utils;
use command_item::CommandItem;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

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

/*
TODO ajustar caixa de ajuda com os comandos
TODO implementar caixa de input + busca pelo input
TODO entender o que causa o bug nas abas de 'namespace'
TODO implementar função de executar os comandos direto da interface
*/

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut command_app = commands::Commands::init();
    let mut namespaces = command_app.namespaces();
    namespaces.insert(0, "All".to_string());
    // create app and run it
    let mut app = App::new(command_app.clone().items, namespaces);

    app.items.command_state.select(Some(0));
    app.items.namespace_state.select(Some(0));

    let res = run_app(&mut terminal, app);

    //restore terminal
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            match key {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                } => return Ok(()),
                KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                }
                | KeyEvent {
                    code: KeyCode::BackTab,
                    modifiers: KeyModifiers::NONE,
                } => app.items.previous_namespace(),
                KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                }
                | KeyEvent {
                    code: KeyCode::Tab,
                    modifiers: KeyModifiers::NONE,
                } => app.items.next_namespace(),
                KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                } => app.items.next(),
                KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                } => app.items.previous(),
                _ => {}
            }
        }
    }

    fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        // Create two chunks with equal horizontal screen space
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(8),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(f.size());

        //create tab menu items
        let tab_menu: Vec<Spans> = app
            .items
            .namespaces
            .clone()
            .into_iter()
            .map(|t| Spans::from(vec![Span::styled(t.clone(), Style::default())]))
            .collect();

        //create tab menu widget
        let tabs = Tabs::new(tab_menu)
            .select(app.items.namespace_state.selected().unwrap())
            .block(Block::default().title(" Namespaces ").borders(Borders::ALL))
            .style(Style::default())
            .highlight_style(
                Style::default()
                    .fg(Color::Rgb(201, 165, 249))
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

                ListItem::new(lines.clone().to_owned()).style(
                    Style::default().fg(Color::Rgb(229, 229, 229)), // .bg(Color::Rgb(229, 229, 229)),
                )
            })
            .collect();

        // Create the command items widget
        let helpitems = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title(" Commands "))
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Rgb(201, 165, 249))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        let idx = app
            .items
            .command_state
            .selected()
            .expect("need to select a current command");

        let selected_command: CommandItem = app.items.filtered_commands().get(idx).unwrap().clone();
        let tags_str = selected_command
            .tags
            .unwrap_or(vec![String::from(" ")])
            .join(", ");
        let command_str: String = selected_command.command;
        let description_str: String = selected_command.description.unwrap_or(String::from(""));
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
        let description = Paragraph::new(description_str)
            .style(Style::default())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default())
                    .title(" Description ")
                    .border_type(BorderType::Plain),
            );

        //Show the current tags
        let tags = Paragraph::new(tags_str)
            .style(Style::default())
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default())
                    .title(" Tags ")
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
                    Constraint::Percentage(50),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(commands_chunks[1]);
        let help_content = "Execute command: <Ctrl-R>       Quit: <Q>       Nada: <Enter>";

        let help = Paragraph::new(help_content).block(
            Block::default()
                .style(Style::default())
                .borders(Borders::ALL)
                .title(" Help ")
                .border_type(BorderType::Plain),
        );

        f.render_widget(help, chunks[2]);
        f.render_widget(command, command_detail_chunks[1]);
        f.render_widget(tags, command_detail_chunks[0]);
        f.render_widget(tabs, chunks[0]);
        f.render_widget(description, command_detail_chunks[2]);
        f.render_stateful_widget(helpitems, commands_chunks[0], &mut app.items.command_state);
    }
}
