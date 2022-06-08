use crate::gui::structs::state::State;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::cursor::set_cursor_positition;

pub fn render<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(frame.size());

    let first_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);
    let second_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    frame.render_widget(create_tags(), first_row[1]);
    frame.render_widget(create_namespace(), second_row[0]);
}

fn create_tags<'a>() -> Paragraph<'a> {
    Paragraph::new("tags")
        .style(
            Style::default()
                .fg(Color::Rgb(229, 229, 229))
                .bg(Color::Rgb(201, 165, 249)),
        )
        .block(Block::default().borders(Borders::ALL).title(" Tags "))
}

fn create_namespace<'a>() -> Paragraph<'a> {
    Paragraph::new("namespace")
        .style(
            Style::default()
                .fg(Color::Rgb(229, 229, 229))
                .bg(Color::Rgb(201, 165, 249)),
        )
        .block(Block::default().borders(Borders::ALL).title(" Namespace "))
}
// fn get_style(component_name: &str, component_state: &Components) -> Style {
//     // info!(
//     //     "validating if actual active component ({}) is equal to given component ({})",
//     //     component_name, state.component_state.active_component.name
//     // );
//     if component_state.active_component.name.eq(component_name) {
//         Style::default()
//             .fg(Color::Rgb(229, 229, 229))
//             .bg(Color::Rgb(201, 165, 249))
//     } else {
//         Style::default().fg(Color::Rgb(229, 229, 229))
//     }
// }
