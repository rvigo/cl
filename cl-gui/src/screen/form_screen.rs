use super::{Screen, ScreenExt};
use crate::{
    default_block,
    entity::{
        context::{ApplicationContext, UI},
        terminal::{TerminalSize, TerminalSizeExt},
    },
    popup, render,
    widget::{
        popup::{HelpPopup, RenderPopup},
        statusbar::{Help, Info, NavigationInfo},
        text_field::FieldType,
    },
};
use tui::{
    layout::{Constraint, Direction, Layout},
    widgets::Block,
    Frame,
};

pub struct FormScreen;

impl Screen for FormScreen {
    fn render(&self, frame: &mut Frame, _: &mut ApplicationContext, ui_context: &mut UI) {
        let block = default_block!(format!(" {} ", ui_context.view_mode()));

        let terminal_size = frame.size().as_terminal_size();

        ui_context.sort_fields(terminal_size.to_owned());

        match terminal_size {
            TerminalSize::Medium => render_medium_form(frame, ui_context, block),
            TerminalSize::Large => render_medium_form(frame, ui_context, block),
            TerminalSize::Small => render_small_form(frame, ui_context, block),
        }

        if ui_context.show_help() {
            let help_popup = popup!(&ui_context.view_mode());
            frame.render_popup(help_popup, frame.size());
        }

        let info = ui_context.popup_info_mut().to_owned();
        if ui_context.show_popup() {
            let popup = popup!(info, ui_context.popup_context_mut().get_available_choices());

            frame.render_stateful_popup(popup, frame.size(), ui_context.popup_context_mut());
        }

        let center = if ui_context.is_form_modified() {
            Some(Info::new("MODIFIED"))
        } else {
            None
        };

        self.render_base(
            frame,
            Some(&NavigationInfo::default()),
            center,
            Some(Help::default()),
        );
    }
}

fn render_medium_form(frame: &mut Frame, ui_context: &mut UI, block: Block) {
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

    render!(frame, {block, form_chunks[0]});

    let fields = ui_context.get_form_fields_iter();
    fields.for_each(|field| {
        let area = match field.field_type() {
            FieldType::Alias => first_row[0],
            FieldType::Namespace => first_row[1],
            FieldType::Command => second_row[0],
            FieldType::Description => third_row[0],
            FieldType::Tags => third_row[1],
        };
        render!(frame, { field, area} );
    })
}

fn render_small_form(frame: &mut Frame, ui_context: &mut UI, block: Block) {
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

    render!(frame, {block, form_chunks[0]});

    let fields = ui_context.get_form_fields_iter();

    fields.for_each(|field| {
        let area = match field.field_type() {
            FieldType::Alias => first_row[0],
            FieldType::Namespace => first_row[1],
            FieldType::Description => second_row[0],
            FieldType::Tags => second_row[1],
            FieldType::Command => third_row[0],
        };
        render!(frame, { field, area} );
    })
}
