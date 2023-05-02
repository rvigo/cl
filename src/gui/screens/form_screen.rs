use super::{
    widgets::{
        help_footer::HelpFooter, help_popup::HelpPopup, navigation_footer::NavigationFooter,
        text_field::FieldType, ScreenExt, WidgetExt,
    },
    Screen, ScreenSize,
};
use crate::{
    centered_rect,
    gui::entities::{
        contexts::{application_context::ApplicationContext, ui_context::UIContext},
        terminal::TerminalSizeExt,
    },
};
use tui::{
    backend::Backend,
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

impl<B> Screen<B> for FormScreen
where
    B: Backend,
{
    fn render(
        &mut self,
        frame: &mut Frame<B>,
        _: &mut ApplicationContext,
        ui_context: &mut UIContext,
    ) {
        let navigation_footer = NavigationFooter::new();
        let help_footer = HelpFooter::new();
        self.render_base(frame, Some(&navigation_footer), help_footer);

        let block = self.default_block(if ui_context.is_form_modified() {
            format!(" {} MODIFIED ", ui_context.view_mode())
        } else {
            format!(" {} ", ui_context.view_mode())
        });
        match self.screen_size {
            ScreenSize::Medium => render_medium_form(frame, ui_context, block),
            ScreenSize::Large => render_medium_form(frame, ui_context, block),
            ScreenSize::Small => render_small_form(frame, ui_context, block),
        }

        if ui_context.show_help() {
            frame.render_widget(
                HelpPopup::new(&ui_context.view_mode(), self.screen_size.to_owned()),
                frame.size(),
            );
        }

        if ui_context.popup().is_some() && ui_context.get_popup_answer().is_none() {
            if let Some(popup) = ui_context.popup() {
                let area = if !ScreenSize::Small.eq(&self.screen_size) {
                    centered_rect!(45, 40, frame.size())
                } else {
                    frame.size()
                };

                frame.render_stateful_widget(popup, area, ui_context.get_choices_state_mut());
            }
        }
    }

    fn set_screen_size(&mut self, screen_size: ScreenSize) {
        self.screen_size = screen_size
    }

    fn get_screen_size(&self) -> ScreenSize {
        self.screen_size.to_owned()
    }
}

impl WidgetExt for FormScreen {}

fn render_medium_form<B>(frame: &mut Frame<B>, ui_context: &mut UIContext, block: Block)
where
    B: Backend,
{
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

fn render_small_form<B>(frame: &mut Frame<B>, ui_context: &UIContext, block: Block)
where
    B: Backend,
{
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
            FieldType::Command => third_row[0],
            FieldType::Description => second_row[0],
            FieldType::Tags => second_row[1],
        };
        frame.render_widget(field.clone(), area);
    })
}
