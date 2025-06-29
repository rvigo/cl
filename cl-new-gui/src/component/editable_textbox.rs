use crate::screen::theme::Theme;
use std::fmt::Display;
use tui::prelude::Style;
use tui::widgets::{Block, Clear};
use tui::Frame;
use tui::layout::Rect;
use tui::style::Color;
use tui_textarea::{CursorMove, TextArea};
use crate::component::Renderable;

#[derive(Default, Debug)]
pub struct EditableTextbox {
    pub name: EditableTextboxName, // TODO see if this field is really needed
    pub textarea: TextArea<'static>,
    pub active: bool,
}

impl EditableTextbox {
    pub fn update_content(&mut self, content: Option<impl Into<String>>) {
        self.textarea
            .insert_str(content.map(|content| content.into()).unwrap_or_default());
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
        if active {
            self.textarea.move_cursor(CursorMove::End);
        }
    }

    pub fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        if self.active {
            self.textarea.input(key);
        }
    }
}

impl Renderable for EditableTextbox {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let theme = theme.to_owned();
        let block = Block::bordered()
            .style(
                Style::default()
                    .fg(theme.clone().text_color.into())
                    .bg(theme.clone().background_color.into()),
            )
            .title(self.name.to_string());
        self.textarea.set_block(block);

        if !self.active {
            self.textarea.set_cursor_style(Style::default());
        }
        else {
            let style = Style::default().bg(Color::Red);
            self.textarea.set_cursor_style(style);
        }

        frame.render_widget(Clear, area);
        frame.render_widget(&self.textarea, area)
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub enum EditableTextboxName {
    Command,
    Description,
    Tags,
    Namespace,
    #[default]
    Alias,
}
impl Display for EditableTextboxName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditableTextboxName::Command => write!(f, "Command"),
            EditableTextboxName::Description => write!(f, "Description"),
            EditableTextboxName::Tags => write!(f, "Tags"),
            EditableTextboxName::Namespace => write!(f, "Namespace"),
            EditableTextboxName::Alias => write!(f, "Alias"),
        }
    }
}
