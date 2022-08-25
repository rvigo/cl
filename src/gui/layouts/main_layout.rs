use super::{cursor::set_cursor_positition, help_layout::render_help, popup_layout::render_popup};
use crate::{
    command::{Command, CommandBuilder},
    gui::{
        entities::{field::Field, popup::Answer, state::State},
        layouts::help_layout::render_helper_footer,
    },
};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
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

    let main_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title(" Command List ")
        .border_type(BorderType::Plain);

    let idx = state
        .commands_state
        .selected()
        .expect("a command should always be selected");

    let selected_command: Command = if state.filter_commands().is_empty() {
        //creates an empty command
        CommandBuilder::default().build()
    } else {
        state.filter_commands().get(idx).unwrap().to_owned()
    };

    state
        .field_context
        .set_current_command(Some(selected_command.clone()));

    let tags_str = selected_command.tags_as_string();

    let command_str: String = selected_command.command.clone();
    let description_str: String = selected_command
        .description
        .unwrap_or_else(|| String::from(""));
    let command = create_command_details(command_str);
    let tabs = create_tab_menu(state);
    let tags = create_tags_menu(tags_str);
    let description = create_command_description(description_str);
    let commands = create_command_items(state);

    let central_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks[2]);

    let command_detail_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Length(3)].as_ref())
        .split(central_chunk[1]);

    let last_line = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[3]);

    frame.render_widget(main_block, frame.size());
    frame.render_widget(tabs, chunks[0]);
    frame.render_stateful_widget(commands, central_chunk[0], &mut state.commands_state);
    frame.render_widget(command, command_detail_chunks[0]);
    frame.render_widget(tags, command_detail_chunks[1]);
    frame.render_widget(description, chunks[1]);
    create_query_box(frame, &mut state.query_box, last_line[0]);
    frame.render_widget(render_helper_footer(), last_line[1]);

    if state.show_help {
        render_help(frame, state)
    }
    if state.popup.show_popup {
        if let Answer::None = state.popup.answer {
            render_popup(frame, state)
        }
    }
}

fn create_query_box<B: Backend>(frame: &mut Frame<B>, query_box: &mut Field, area: Rect) {
    let mut query_string;
    if !query_box.in_focus() && query_box.input.is_empty() {
        query_string = String::from("Press <f> to find commands")
    } else {
        query_string = query_box.input.clone()
    }

    if query_box.in_focus() {
        query_string = query_box.input.clone();
        set_cursor_positition(frame, query_box, area);
    }

    let query_box = Paragraph::new(query_string)
        .style(if query_box.in_focus() {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(201, 165, 249))
        } else if !query_box.in_focus() && !query_box.input.is_empty() {
            Style::default().fg(Color::Rgb(201, 165, 249))
        } else {
            Style::default().fg(Color::Rgb(229, 229, 229))
        })
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(if !query_box.in_focus() {
                    Style::default().fg(Color::Rgb(229, 229, 229))
                } else {
                    Style::default()
                })
                .title(query_box.title())
                .border_type(BorderType::Plain),
        );

    frame.render_widget(query_box, area);
}

fn create_tab_menu<'a>(state: &State) -> Tabs<'a> {
    let tab_menu: Vec<Spans> = state
        .namespaces
        .clone()
        .into_iter()
        .map(|tab| Spans::from(vec![Span::styled(tab, Style::default())]))
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
        .filter_commands()
        .into_iter()
        .map(|c| {
            let lines = vec![Spans::from(c.alias)];

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