use super::cursor::set_cursor_positition;
use crate::gui::entities::{field::Field, fields_context::FieldsContext, state::State};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub const DEFAULT_TEXT_COLOR: Color = Color::Rgb(229, 229, 229);
pub const DEFAULT_SELECTED_COLOR: Color = Color::Rgb(201, 165, 249);

pub fn render_widget<B: Backend>(frame: &mut Frame<B>, state: &State, area: Rect, field: &Field) {
    let widget = Paragraph::new(state.field_context.get_component_input(field.name()))
        .style(get_style(&state.field_context, field.name()))
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
    if state.field_context.is_in_focus(field.name()) {
        set_cursor_positition(
            frame,
            state.field_context.get_current_in_focus().unwrap(),
            area,
        )
    }
}

fn get_style(fields_context: &FieldsContext, component_name: &str) -> Style {
    if fields_context.is_in_focus(component_name) {
        Style::default().fg(Color::Black).bg(DEFAULT_SELECTED_COLOR)
    } else {
        Style::default().fg(DEFAULT_TEXT_COLOR)
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
