use std::sync::Arc;

use super::{centered_rect, TerminalSize};
use crate::gui::{
    entities::{application_context::ApplicationContext, ui_state::UiState},
    widgets::{
        base_widget::BaseWidget, field::FieldType, help_footer::HelpFooter, help_popup::HelpPopup,
        navigation_footer::NavigationFooter,
    },
};
use parking_lot::Mutex;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub fn render<B: Backend>(
    frame: &mut Frame<B>,
    application_context: &mut Arc<Mutex<ApplicationContext>>,
    ui_state: &UiState,
) {
    render_base_widget(frame);
    match ui_state.size {
        TerminalSize::Medium => render_medium_form(frame, application_context, ui_state),
        TerminalSize::Large => render_medium_form(frame, application_context, ui_state),
        TerminalSize::Small => render_small_form(frame, application_context, ui_state),
    }

    render_popup(frame, application_context, ui_state)
}

fn render_popup<B: Backend>(
    frame: &mut Frame<B>,
    context: &mut Arc<Mutex<ApplicationContext>>,
    ui_state: &UiState,
) {
    let mut context = context.lock();
    if context.show_help() {
        frame.render_widget(
            HelpPopup::new(ui_state.view_mode.clone(), ui_state.size.clone()),
            frame.size(),
        );
    }

    if context.popup().is_some() && context.get_popup_answer().is_none() {
        let popup = &context.popup().unwrap().clone();

        let area = if !TerminalSize::Small.eq(&ui_state.size) {
            centered_rect(45, 40, frame.size())
        } else {
            frame.size()
        };

        frame.render_stateful_widget(popup.to_owned(), area, context.get_choices_state_mut());
    }
}

fn render_base_widget<B: Backend>(frame: &mut Frame<B>) {
    let navigation_footer = NavigationFooter::new();
    let help_footer = HelpFooter::new();
    let base_widget = BaseWidget::new(
        &super::TerminalSize::Medium,
        Some(&navigation_footer),
        help_footer,
    );
    frame.render_widget(base_widget, frame.size());
}

fn render_medium_form<B: Backend>(
    frame: &mut Frame<B>,
    application_context: &mut Arc<Mutex<ApplicationContext>>,
    ui_state: &UiState,
) {
    let context = application_context.lock();
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
        .title(format!(" {} ", ui_state.view_mode))
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
    application_context: &mut Arc<Mutex<ApplicationContext>>,
    ui_state: &UiState,
) {
    let context = application_context.lock();

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
        .title(format!(" {} ", ui_state.view_mode))
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

    let fields = context.get_form_fields();

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
