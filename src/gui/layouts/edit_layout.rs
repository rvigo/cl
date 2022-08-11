use super::{
    help_layout::{render_help, render_helper_footer},
    layout_utils::render_widget,
    popup_layout::render_popup,
};
use crate::gui::entities::{field::FieldType, state::State};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub fn render<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Max(3),
            ]
            .as_ref(),
        )
        .split(frame.size());
    let edit_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title(" Edit ")
        .border_type(BorderType::Plain);
    let first_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);
    let second_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);
    let third_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);
    let fourth_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[3]);

    frame.render_widget(edit_block, frame.size());
    for item in state.context.fields().iter() {
        match item.field_type() {
            FieldType::Tags => render_widget(frame, state, third_row[1], item),
            FieldType::Namespace => render_widget(frame, state, first_row[1], item),
            FieldType::Alias => render_widget(frame, state, first_row[0], item),
            FieldType::Command => render_widget(frame, state, second_row[0], item),
            FieldType::Description => render_widget(frame, state, third_row[0], item),
            _ => {}
        }
    }

    frame.render_widget(render_helper_footer(), fourth_row[0]);

    if state.show_help {
        render_help(frame, state)
    }
    if state.popup.show_popup {
        render_popup(frame, state);
    }
}
