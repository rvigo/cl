use crate::gui::{
    entities::{field::Field, fields_context::FieldsContext, state::State},
    key_handlers::cursor::set_cursor_positition,
};
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

pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let new_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - height) / 2),
                Constraint::Percentage(height),
                Constraint::Percentage((100 - height) / 2),
            ]
            .as_ref(),
        )
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - width) / 2),
                Constraint::Percentage(width),
                Constraint::Percentage((100 - width) / 2),
            ]
            .as_ref(),
        )
        .split(new_area[1])[1]
}
