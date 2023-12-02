use super::{Screen, ScreenExt, ScreenSize};
use crate::{
    centered_rect,
    entities::{
        contexts::{application_context::ApplicationContext, ui_context::UIContext},
        terminal::TerminalSizeExt,
    },
    widgets::{
        popup::{
            help_popup::HelpPopup, option::Choice, question_popup::QuestionPopup, RenderPopup,
        },
        statusbar::{help::Help, info::Info, navigation_info::NavigationInfo},
        text_field::FieldType,
        WidgetExt,
    },
};
use tui::{
    layout::{Constraint, Direction, Layout},
    widgets::Block,
    Frame,
};

pub struct FormScreen {
    screen_size: ScreenSize,
}

impl FormScreen {
    pub fn new(screen_size: ScreenSize) -> Self {
        Self { screen_size }
    }
}

impl WidgetExt for FormScreen {}

impl Screen for FormScreen {
    fn render(
        &mut self,
        frame: &mut Frame,
        _: &mut ApplicationContext,
        ui_context: &mut UIContext,
    ) {
        let block = self.default_block(format!(" {} ", ui_context.view_mode()));

        let screen_size = frame.size().as_terminal_size().into();

        if screen_size != self.screen_size {
            <FormScreen as Screen>::set_screen_size(self, screen_size);
        }

        match self.screen_size {
            ScreenSize::Medium => render_medium_form(frame, ui_context, block),
            ScreenSize::Large => render_medium_form(frame, ui_context, block),
            ScreenSize::Small => render_small_form(frame, ui_context, block),
        }

        if ui_context.show_help() {
            let hp = HelpPopup::new(&ui_context.view_mode());
            frame.render_popup(hp, frame.size());
        }

        if ui_context.show_popup() {
            let area = if !ScreenSize::Small.eq(&self.screen_size) {
                centered_rect!(45, 40, frame.size())
            } else {
                frame.size()
            };

            let p = QuestionPopup::new(
                ui_context.popup_container.message.clone(),
                Choice::dialog(),
                ui_context.popup_container.popup_type.to_owned(),
            );
            frame.render_stateful_popup(p, area, ui_context.get_choices_state_mut());
        }

        let center = if ui_context.is_form_modified() {
            Some(Info::new("MODIFIED"))
        } else {
            None
        };

        let help = Help::new();

        self.render_base(frame, Some(&NavigationInfo::new()), center, Some(help));
    }

    fn set_screen_size(&mut self, screen_size: ScreenSize) {
        self.screen_size = screen_size
    }

    fn get_screen_size(&self) -> ScreenSize {
        self.screen_size.to_owned()
    }
}

fn render_medium_form(frame: &mut Frame, ui_context: &mut UIContext, block: Block) {
    ui_context.update_screen_size(frame.size().as_terminal_size());

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

    frame.render_widget(block, chunks[0]);

    let fields = ui_context.get_form_fields_iter();
    fields.for_each(|field| {
        let area = match field.field_type() {
            FieldType::Alias => first_row[0],
            FieldType::Namespace => first_row[1],
            FieldType::Command => second_row[0],
            FieldType::Description => third_row[0],
            FieldType::Tags => third_row[1],
        };
        frame.render_widget(field.clone(), area);
    })
}

fn render_small_form(frame: &mut Frame, ui_context: &mut UIContext, block: Block) {
    ui_context.update_screen_size(frame.size().as_terminal_size());

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

    frame.render_widget(block, form_chunks[0]);

    let fields = ui_context.get_form_fields_iter();

    fields.for_each(|field| {
        let area = match field.field_type() {
            FieldType::Alias => first_row[0],
            FieldType::Namespace => first_row[1],
            FieldType::Description => second_row[0],
            FieldType::Tags => second_row[1],
            FieldType::Command => third_row[0],
        };
        frame.render_widget(field.clone(), area);
    })
}
