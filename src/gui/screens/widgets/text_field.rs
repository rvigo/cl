use super::{WidgetExt, WidgetKeyHandler};
use crate::gui::DEFAULT_TEXT_COLOR;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    slice::Iter,
};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::Widget,
};
use tui_textarea::{
    CursorMove::{Bottom, End},
    TextArea,
};

#[derive(Debug, Clone, Eq, Hash, PartialEq, Default)]
pub enum FieldType {
    #[default]
    Alias,
    Tags,
    Command,
    Description,
    Namespace,
}

impl FieldType {
    pub fn iter() -> Iter<'static, FieldType> {
        [
            FieldType::Alias,
            FieldType::Tags,
            FieldType::Command,
            FieldType::Description,
            FieldType::Namespace,
        ]
        .iter()
    }
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

/// Represents a text field component/widget
#[derive(Clone)]
pub struct TextField<'a> {
    title: String,
    field_type: FieldType,
    in_focus: bool,
    alignment: Alignment,
    multiline: bool,
    text_area: TextArea<'a>,
}

impl<'a> TextField<'a> {
    pub fn new<T>(title: T, field_type: FieldType, in_focus: bool, multiline: bool) -> TextField<'a>
    where
        T: Into<String>,
    {
        TextField {
            title: title.into(),
            field_type,
            in_focus,
            alignment: Alignment::Left,
            multiline,
            text_area: TextArea::default(),
        }
    }

    pub fn field_type(&self) -> FieldType {
        self.field_type.to_owned()
    }

    pub fn text(&self) -> String {
        self.text_area.to_owned().into_lines().join("\n")
    }

    pub fn set_text<L>(&mut self, content: L)
    where
        L: ToLines,
    {
        self.text_area = TextArea::from(content.to_lines())
    }

    pub fn activate_focus(&mut self) {
        self.in_focus = true
    }

    pub fn deactivate_focus(&mut self) {
        self.in_focus = false
    }

    pub fn in_focus(&self) -> bool {
        self.in_focus
    }

    pub fn move_cursor_to_end_of_text(&mut self) {
        self.text_area.move_cursor(Bottom);
        self.text_area.move_cursor(End);
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

impl<'a> Drop for TextField<'a> {
    fn drop(&mut self) {
        self.clear_input()
    }
}

impl WidgetKeyHandler for TextField<'_> {
    fn handle_input(&mut self, input: KeyEvent) {
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
}

impl<'a> Widget for TextField<'a> {
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
        let title = self.title.clone();
        let default_block = self.default_block(title);

        self.text_area.set_block(default_block);

        self.text_area.set_cursor_line_style(Style::default());
        self.text_area.set_alignment(self.alignment);
        self.text_area.set_style(self.get_style(self.in_focus));
        self.text_area.widget().render(area, buf)
    }
}

type Lines = Vec<String>;

/// Converts the content to `Lines`
pub trait ToLines {
    fn to_lines(&self) -> Lines;
}

impl ToLines for Vec<String> {
    fn to_lines(&self) -> Lines {
        self.to_owned()
    }
}

impl ToLines for String {
    fn to_lines(&self) -> Lines {
        vec![self.to_owned()]
    }
}
