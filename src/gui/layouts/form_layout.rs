use crate::gui::{
    entities::state::State,
    widgets::{base_widget::BaseWidget, field::FieldType, help_popup::HelpPopup},
};
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

    frame.render_widget(BaseWidget::new(None), frame.size());
    frame.render_widget(form_block, chunks[0]);

    for field in state.form_fields_context.fields.clone().into_iter() {
        match field.field_type {
            FieldType::Alias => frame.render_widget(field, first_row[0]),
            FieldType::Namespace => frame.render_widget(field, first_row[1]),
            FieldType::Command => frame.render_widget(field, second_row[0]),
            FieldType::Description => frame.render_widget(field, third_row[0]),
            FieldType::Tags => frame.render_widget(field, third_row[1]),
        }
    }

    if state.show_help {
        frame.render_widget(HelpPopup::new(state.view_mode.clone()), frame.size());
    }
    if state.popup_context.popup.is_some() && state.popup_context.answer.is_none() {
        if let Some(popup) = &state.popup_context.popup {
            let popup = popup.clone();
            frame.render_stateful_widget(
                popup,
                frame.size(),
                &mut state.popup_context.choices_state,
            );
        }
    }
}
