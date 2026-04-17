use crate::component::Renderable;
use crate::screen::theme::Theme;
use crate::state::state_event::FieldName;
use tui::layout::Rect;
use tui::prelude::Style;
use tui::widgets::{Block, Clear};
use tui::Frame;
use tui_textarea::{CursorMove, TextArea};

#[derive(Default, Debug)]
pub struct EditableTextbox {
    pub name: FieldName,
    pub textarea: TextArea<'static>,
    pub active: bool,
    pub modified: bool,
}

impl EditableTextbox {
    pub fn update_content(&mut self, content: Option<impl Into<String>>) {
        self.textarea.select_all();
        self.textarea.cut();
        if let Some(c) = content {
            self.textarea.insert_str(c.into());
        }
        self.modified = false;
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
        let block = Block::bordered()
            .style(
                Style::default()
                    .fg(theme.text_color.into())
                    .bg(theme.background_color.into()),
            )
            .title(if self.modified {
                format!("{} *", self.name)
            } else {
                self.name.to_string()
            });
        self.textarea.set_block(block);

        if self.active {
            self.textarea
                .set_cursor_style(Style::default().bg(theme.cursor_color.into()));
        } else {
            self.textarea.set_cursor_style(Style::default());
        }

        frame.render_widget(Clear, area);
        frame.render_widget(&self.textarea, area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_content_replaces_previous_value() {
        let mut tb = EditableTextbox::default();
        tb.update_content(Some("first"));
        tb.update_content(Some("second"));
        let lines = tb.textarea.lines().join("\n");
        assert_eq!(lines, "second");
    }

    #[test]
    fn update_content_resets_modified_flag() {
        let mut tb = EditableTextbox::default();
        tb.modified = true;
        tb.update_content(Some("value"));
        assert!(!tb.modified);
    }

    #[test]
    fn update_content_with_none_clears_textarea() {
        let mut tb = EditableTextbox::default();
        tb.update_content(Some("something"));
        tb.update_content(None::<String>);
        let lines = tb.textarea.lines().join("\n");
        assert!(lines.is_empty());
    }
}

impl crate::observer::event::NotifyTarget for EditableTextbox {
    type Payload = crate::observer::event::EditableTextboxEvent;
    fn wrap(payload: Self::Payload) -> crate::observer::event::Event {
        crate::observer::event::Event::EditableTextbox(payload)
    }
}
