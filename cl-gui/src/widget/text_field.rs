use super::{Lines, WidgetKeyHandler};
use crate::{
    DEFAULT_BACKGROUND_COLOR, DEFAULT_CURSOR_COLOR, DEFAULT_HIGH_LIGHT_COLOR,
    DEFAULT_INACTIVE_TEXTBOX_COLOR, DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{
    fmt::{Debug, Display},
    hash::Hash,
};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::{Block, BorderType, Borders, Padding, Widget},
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
    pub fn values() -> [FieldType; 5] {
        [
            FieldType::Alias,
            FieldType::Tags,
            FieldType::Command,
            FieldType::Description,
            FieldType::Namespace,
        ]
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
pub struct TextField<'txt> {
    title: String,
    field_type: FieldType,
    pub in_focus: bool,
    alignment: Alignment,
    multiline: bool,
    text_area: TextArea<'txt>,
}

impl<'txt> TextField<'txt> {
    pub fn new<T>(
        title: T,
        field_type: FieldType,
        in_focus: bool,
        multiline: bool,
    ) -> TextField<'txt>
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
        L: Into<Lines>,
    {
        self.text_area = TextArea::from(&*content.into())
    }

    pub fn lines(&self) -> Vec<String> {
        self.text_area.to_owned().into_lines()
    }

    pub fn activate_focus(&mut self) {
        self.in_focus = true
    }

    pub fn deactivate_focus(&mut self) {
        self.in_focus = false
    }

    pub fn move_cursor_to_end_of_text(&mut self) {
        self.text_area.move_cursor(Bottom);
        self.text_area.move_cursor(End);
    }

    pub fn clear_input(&mut self) {
        self.text_area = self.default_text_area()
    }

    fn default_text_area(&self) -> TextArea<'txt> {
        let mut text_area = TextArea::default();
        text_area.set_cursor_line_style(Style::default());
        text_area.set_cursor_style(
            Style::default()
                .fg(DEFAULT_TEXT_COLOR)
                .add_modifier(Modifier::REVERSED),
        );
        text_area
    }

    fn text_area_style(&self) -> Style {
        if self.in_focus {
            Style::default()
                .fg(DEFAULT_TEXT_COLOR)
                .bg(DEFAULT_SELECTED_COLOR)
        } else {
            Style::default().fg(DEFAULT_TEXT_COLOR)
        }
    }

    fn title_style(&self) -> Style {
        if self.in_focus {
            Style::default()
                .fg(DEFAULT_HIGH_LIGHT_COLOR)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC)
        } else {
            Style::default()
                .fg(DEFAULT_INACTIVE_TEXTBOX_COLOR)
                .add_modifier(Modifier::BOLD)
        }
    }

    fn cursor_style(&self) -> Style {
        if self.in_focus {
            Style::default()
                .fg(DEFAULT_CURSOR_COLOR)
                .add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        }
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
        self.text_area.set_cursor_style(self.cursor_style());
        let block = Block::default()
            .borders(Borders::TOP | Borders::RIGHT)
            .title(format!(" {} ", self.title))
            .title_alignment(Alignment::Left)
            .title_style(self.title_style())
            .style(
                Style::default()
                    .fg(DEFAULT_TEXT_COLOR)
                    .bg(DEFAULT_BACKGROUND_COLOR),
            )
            .border_type(BorderType::Rounded)
            .padding(Padding::horizontal(2));

        self.text_area.set_block(block);

        self.text_area.set_cursor_line_style(Style::default());
        self.text_area.set_alignment(self.alignment);
        self.text_area.set_style(self.text_area_style());

        self.text_area.widget().render(area, buf)
    }
}

#[derive(Default)]
pub struct TextFieldBuilder {
    field_type: FieldType,
    in_focus: bool,
    multiline: bool,
}

impl TextFieldBuilder {
    pub fn field_type(mut self, field_type: FieldType) -> Self {
        self.field_type = field_type;
        self
    }

    pub fn in_focus(mut self, in_focus: bool) -> Self {
        self.in_focus = in_focus;
        self
    }

    pub fn multiline(mut self, multiline: bool) -> Self {
        self.multiline = multiline;
        self
    }

    pub fn build(self) -> TextField<'static> {
        let title = self.field_type.to_string();
        TextField::new(title, self.field_type, self.in_focus, self.multiline)
    }
}
