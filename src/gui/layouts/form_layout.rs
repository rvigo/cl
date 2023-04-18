use super::{centered_rect, get_default_block, get_forms_main_block, TerminalSize};
use crate::gui::{
    entities::contexts::ui_context::UIContext,
    widgets::{
        base_widget::BaseWidget, help_footer::HelpFooter, help_popup::HelpPopup,
        navigation_footer::NavigationFooter, text_field::FieldType,
    },
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub fn render<B: Backend>(frame: &mut Frame<B>, ui_context: &mut UIContext) {
    render_base_widget(frame);
    match ui_context.terminal_size() {
        TerminalSize::Medium => render_medium_form(frame, ui_context),
        TerminalSize::Large => render_medium_form(frame, ui_context),
        TerminalSize::Small => render_small_form(frame, ui_context),
    }

    render_popup(frame, ui_context)
}

fn render_popup<B: Backend>(frame: &mut Frame<B>, ui_context: &mut UIContext) {
    if ui_context.show_help() {
        frame.render_widget(
            HelpPopup::new(ui_context.view_mode(), ui_context.terminal_size().clone()),
            frame.size(),
        );
    }

    if ui_context.popup().is_some() && ui_context.get_popup_answer().is_none() {
        let popup = &ui_context.popup().unwrap();

        let area = if !TerminalSize::Small.eq(ui_context.terminal_size()) {
            centered_rect(45, 40, frame.size())
        } else {
            frame.size()
        };

        frame.render_stateful_widget(popup.to_owned(), area, ui_context.get_choices_state_mut());
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

fn render_medium_form<B: Backend>(frame: &mut Frame<B>, ui_context: &UIContext) {
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

    let form_block = get_forms_main_block(
        ui_context.view_mode().to_string(),
        ui_context.is_form_modified(),
    );

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

    let fields = &(*ui_context.get_form_fields());

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

fn render_small_form<B: Backend>(frame: &mut Frame<B>, ui_context: &UIContext) {
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

    let form_block = get_default_block(ui_context.view_mode().to_string());
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

    let fields = ui_context.get_form_fields();

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
