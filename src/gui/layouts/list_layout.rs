use crate::command_item::CommandItem;
use crate::gui::structs::state::State;

use log::info;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn render<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let idx = state
        .commands_state
        .selected()
        .expect("a command should always be selected");
    info!("got idx {idx}");

    let mut selected_command: CommandItem = state.filtered_commands().get(idx).unwrap().clone();
    state.current_command = Some(selected_command.clone());

    let tags_str = selected_command.tags_str();

    //renewing state
    let state = state.get_mut_ref();

    let command_str: String = selected_command.command;
    let description_str: String = selected_command.description.unwrap_or(String::from(""));

    let command = create_command_details(command_str);
    let tabs = create_tab_menu(state.get_ref());
    let tags = create_tags_menu(tags_str);
    let description = create_command_description(description_str);
    let commands = create_command_items(state.get_mut_ref());

    let central_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[2]);

    let command_detail_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Length(3)].as_ref())
        .split(central_chunk[1]);

    let help_content = "Quit: <Q>       New command: <Ins>";

    let help = Paragraph::new(help_content).block(
        Block::default()
            .style(Style::default())
            .borders(Borders::ALL)
            .title(" Help ")
            .border_type(BorderType::Plain),
    );

    let command_state = state.get_command_state_mut_ref();

    frame.render_widget(tabs, chunks[0]);
    frame.render_stateful_widget(commands, central_chunk[0], command_state);
    frame.render_widget(command, command_detail_chunks[0]);
    frame.render_widget(tags, command_detail_chunks[1]);
    frame.render_widget(description, chunks[1]);
    frame.render_widget(help, chunks[3]);
}

fn create_tab_menu<'a>(state: &State) -> Tabs<'a> {
    let tab_menu: Vec<Spans> = state
        .namespaces
        .clone()
        .into_iter()
        .map(|tab| Spans::from(vec![Span::styled(tab.clone(), Style::default())]))
        .collect();
    Tabs::new(tab_menu)
        .select(state.namespace_state.selected().unwrap())
        .block(Block::default().title(" Namespaces ").borders(Borders::ALL))
        .style(Style::default())
        .highlight_style(
            Style::default()
                .fg(Color::Rgb(201, 165, 249))
                .add_modifier(Modifier::UNDERLINED),
        )
        .divider(Span::raw("|"))
}

fn create_command_items<'a>(state: &mut State) -> List<'a> {
    let list_items: Vec<ListItem> = state
        .filtered_commands()
        .into_iter()
        .map(|c| {
            let lines = vec![Spans::from(c.alias.clone().unwrap_or(String::from("-")))];

            ListItem::new(lines.clone().to_owned())
                .style(Style::default().fg(Color::Rgb(229, 229, 229)))
        })
        .collect();

    List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(" Aliases "))
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(201, 165, 249))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ")
}

fn create_command_details<'a>(command: String) -> Paragraph<'a> {
    Paragraph::new(command)
        .style(Style::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title(" Command ")
                .border_type(BorderType::Plain),
        )
}

fn create_command_description<'a>(description: String) -> Paragraph<'a> {
    Paragraph::new(description)
        .style(Style::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title(" Description ")
                .border_type(BorderType::Plain),
        )
}

fn create_tags_menu<'a>(tags: String) -> Paragraph<'a> {
    Paragraph::new(tags)
        .style(Style::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title(" Tags ")
                .border_type(BorderType::Plain),
        )
}
