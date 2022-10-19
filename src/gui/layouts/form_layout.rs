use super::{
    help_layout::{render_help, render_helper_footer},
    layout_utils::{get_main_block, render_widget},
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
                Constraint::Min(10),   //Form
                Constraint::Length(3), //Help
            ]
            .as_ref(),
        )
        .split(frame.size());
    let form_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(5), //Alias & Namespace
                Constraint::Min(10),   //Command
                Constraint::Length(5), //Desc & Tags
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    let form_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title(format!(" {} ", state.view_mode))
        .border_type(BorderType::Plain);
    let first_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(form_chunks[0]);
    let second_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(form_chunks[1]);
    let third_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(form_chunks[2]);
    let fourth_row = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)].as_ref())
        .split(chunks[1]);

    frame.render_widget(get_main_block(), frame.size());
    frame.render_widget(form_block, chunks[0]);

    for field in state.field_context.fields().iter() {
        match field.field_type() {
            FieldType::Alias => render_widget(frame, state, first_row[0], field),
            FieldType::Namespace => render_widget(frame, state, first_row[1], field),
            FieldType::Command => render_widget(frame, state, second_row[0], field),
            FieldType::Description => render_widget(frame, state, third_row[0], field),
            FieldType::Tags => render_widget(frame, state, third_row[1], field),
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
