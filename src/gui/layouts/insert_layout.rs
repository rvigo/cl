use crate::gui::structs::state::State;
use log::info;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use super::cursor::set_cursor_positition;

pub fn render<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
        .split(frame.size());

    let first_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);
    let second_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let tags = create_tags(state);
    let namespace = create_namespace(state);
    frame.render_widget(tags, first_row[1]);
    frame.render_widget(namespace, second_row[0]);
}

fn create_tags<'a>(state: &mut State) -> Paragraph<'a> {
    let component_name = "tags";
    Paragraph::new(
        state
            .get_mut_ref()
            .focus
            .get_component_input(component_name),
    )
    .style(get_style(state, component_name))
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

fn create_namespace<'a>(state: &mut State) -> Paragraph<'a> {
    let component_name = "namespace";

    Paragraph::new(
        state
            .get_mut_ref()
            .focus
            .get_component_input(component_name),
    )
    .style(get_style(state.get_mut_ref(), component_name))
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true })
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(" Namespace ")
            .border_type(BorderType::Plain),
    )
}

fn get_style(state: &mut State, component_name: &str) -> Style {
    if state.focus.is_in_focus(component_name) {
        Style::default()
            .fg(Color::Rgb(229, 229, 229))
            .bg(Color::Rgb(201, 165, 249))
    } else {
        Style::default().fg(Color::Rgb(229, 229, 229))
    }
}
