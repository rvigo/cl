use crate::gui::layouts::{get_default_block, get_style, DEFAULT_TEXT_COLOR};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::Widget,
};
use tui_textarea::TextArea;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Default)]
pub enum FieldType {
    #[default]
    Alias,
    Tags,
    Command,
    Description,
    Namespace,
}

impl Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Alias => write!(f, "Alias"),
            FieldType::Tags => write!(f, "Tags"),
            FieldType::Command => write!(f, "Command"),
            FieldType::Description => write!(f, "Description"),
            FieldType::Namespace => write!(f, "Namespace"),
        }
    }
}

#[derive(Clone)]
pub struct Field<'a> {
    title: String,
    pub field_type: FieldType,
    in_focus: bool,
    alignment: Alignment,
    pub multiline: bool,
    pub text_area: TextArea<'a>,
}

impl<'a> Field<'a> {
    pub fn new<T>(title: T, field_type: FieldType, in_focus: bool, multiline: bool) -> Field<'a>
    where
        T: Into<String>,
    {
        Field {
            title: title.into(),
            field_type,
            in_focus,
            alignment: Alignment::Left,
            multiline,
            text_area: TextArea::default(),
        }
    }

    pub fn activate_focus(&mut self) {
        self.in_focus = true
    }

    pub fn deactivate_focus(&mut self) {
        self.in_focus = false
    }

    pub fn input_as_string(&mut self) -> String {
        self.text_area.to_owned().into_lines().join("\n")
    }

    pub fn in_focus(&self) -> bool {
        self.in_focus
    }

    pub fn on_input(&mut self, input: KeyEvent) {
        if self.multiline {
            self.text_area.input(input);
        } else {
            // should avoid new lines
            match input {
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    ..
                }
                | KeyEvent {
                    code: KeyCode::Char('m'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {}
                input => {
                    self.text_area.input(input);
                }
            }
        }
    }

    pub fn clear_input(&mut self) {
        self.text_area = self.default_text_area()
    }

    fn default_text_area(&self) -> TextArea<'a> {
        let mut text_area = TextArea::default();
        text_area.set_cursor_line_style(Style::default());
        text_area.set_cursor_style(
            Style::default()
                .fg(DEFAULT_TEXT_COLOR)
                .add_modifier(Modifier::REVERSED),
        );
        text_area
    }
}

impl<'a> Drop for Field<'a> {
    fn drop(&mut self) {
        self.clear_input()
    }
}

impl<'a> Widget for Field<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        if self.in_focus() {
            self.text_area.set_cursor_style(
                Style::default()
                    .fg(DEFAULT_TEXT_COLOR)
                    .add_modifier(Modifier::REVERSED),
            );
        } else {
            self.text_area.set_cursor_style(Style::default());
        };
        self.text_area.set_block(get_default_block(&self.title));

        self.text_area.set_cursor_line_style(Style::default());
        self.text_area.set_alignment(self.alignment);
        self.text_area.set_style(get_style(self.in_focus));
        self.text_area.widget().render(area, buf)
    }
}
