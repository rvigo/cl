use super::cursor::set_cursor_positition;
use crate::gui::entities::{field::Field, state::State};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_widget<B: Backend>(frame: &mut Frame<B>, state: &State, area: Rect, field: &Field) {
    let widget = Paragraph::new(state.context.get_component_input(field.name()))
        .style(get_style(state, field.name()))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title(field.title())
                .border_type(BorderType::Plain),
        );

    frame.render_widget(widget, area);
    if state.context.is_in_focus(field.name()) {
        set_cursor_positition(frame, state.context.get_current_in_focus().unwrap(), area)
    }
}

fn get_style(state: &State, component_name: &str) -> Style {
    if state.context.is_in_focus(component_name) {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Rgb(201, 165, 249))
    } else {
        Style::default().fg(Color::Rgb(229, 229, 229))
    }
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Vec<Rect> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(layout[1])
}
