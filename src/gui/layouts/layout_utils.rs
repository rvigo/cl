use super::cursor::set_cursor_positition;
use crate::gui::entities::{field::Field, state::State};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_widget<B: Backend>(frame: &mut Frame<B>, state: &State, area: Rect, item: &Field) {
    let widget = Paragraph::new(state.context.get_component_input(item.name()))
        .style(get_style(state, item.name()))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title(item.title())
                .border_type(BorderType::Plain),
        );

    frame.render_widget(widget, area);
    if state.context.is_in_focus(item.name()) {
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
