use super::{centered_rect, TerminalSize};
use crate::gui::{
    entities::application_context::ApplicationContext,
    widgets::{base_widget::BaseWidget, field::FieldType, help_popup::HelpPopup},
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub fn render<B: Backend>(
    frame: &mut Frame<B>,
    context: &mut ApplicationContext,
    terminal_size: TerminalSize,
) {
    render_base_widget(frame);

    match terminal_size {
        TerminalSize::Medium => render_medium_form(frame, context),
        TerminalSize::Large => render_medium_form(frame, context),
        TerminalSize::Small => render_small_form(frame, context),
    }

    if context.show_help() {
        frame.render_widget(
            HelpPopup::new(context.view_mode().clone(), terminal_size.clone()),
            frame.size(),
        );
    }

    if context.popup().is_some() && context.get_popup_answer().is_none() {
        let popup = &context.popup().unwrap().clone();

        let area = if terminal_size != TerminalSize::Small {
            centered_rect(45, 40, frame.size())
        } else {
            frame.size()
        };

        frame.render_stateful_widget(popup.to_owned(), area, context.get_choices_state_mut());
    }
}

fn render_base_widget<B: Backend>(frame: &mut Frame<B>) {
    let base_widget = BaseWidget::new(None, &super::TerminalSize::Medium);
    frame.render_widget(base_widget, frame.size());
}

fn render_medium_form<B: Backend>(frame: &mut Frame<B>, context: &mut ApplicationContext) {
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
        .title(format!(" {} ", context.view_mode()))
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

    frame.render_widget(form_block, chunks[0]);

    let fields = &(*context.get_form_fields()).clone();

    fields.iter().for_each(|field| {
        let area = match field.field_type {
            FieldType::Alias => first_row[0],
            FieldType::Namespace => first_row[1],
            FieldType::Command => second_row[0],
            FieldType::Description => third_row[0],
            FieldType::Tags => third_row[1],
        };
        frame.render_widget(field.clone(), area);
    })
}

fn render_small_form<B: Backend>(
    frame: &mut Frame<B>,
    application_context: &mut ApplicationContext,
) {
    let form_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), //Alias & Namespace
                Constraint::Length(3), //Desc & Tags
                Constraint::Min(7),    //Command,
            ]
            .as_ref(),
        )
        .split(frame.size());

    let form_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title(format!(" {} ", application_context.view_mode()))
        .border_type(BorderType::Plain);
    let first_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(form_chunks[0]);
    let second_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(form_chunks[1]);
    let third_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(form_chunks[2]);

    frame.render_widget(form_block, form_chunks[0]);

    let fields = application_context.get_form_fields();

    fields.iter().for_each(|field| {
        let area = match field.field_type {
            FieldType::Alias => first_row[0],
            FieldType::Namespace => first_row[1],
            FieldType::Command => third_row[0],
            FieldType::Description => second_row[0],
            FieldType::Tags => second_row[1],
        };
        frame.render_widget(field.clone(), area);
    })
}
