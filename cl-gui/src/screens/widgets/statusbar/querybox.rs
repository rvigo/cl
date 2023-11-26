use super::StatusBarItem;
use crate::{screens::widgets::WidgetKeyHandler, DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR};
use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Widget},
};
use tui_textarea::TextArea;

#[derive(Clone, Default)]
pub struct QueryBox<'a> {
    text_area: TextArea<'a>,
    on_focus: bool,
    buffer: String,
}

impl<'a> StatusBarItem for QueryBox<'a> {}

impl<'a> QueryBox<'a> {
    pub fn activate_focus(&mut self) {
        self.on_focus = true
    }

    pub fn deactivate_focus(&mut self) {
        self.on_focus = false
    }

    pub fn get_input(&self) -> String {
        self.buffer.to_owned()
    }
}

impl WidgetKeyHandler for QueryBox<'_> {
    fn handle_input(&mut self, key_event: KeyEvent) {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc | KeyCode::Enter | KeyCode::Down | KeyCode::Up,
                ..
            } => self.on_focus = false,
            input => {
                self.text_area.input(input);
                self.buffer = self.text_area.lines()[0].clone()
            }
        }
    }
}

impl<'a> Widget for QueryBox<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let style = if self.on_focus {
            Style::default().fg(Color::Black).bg(DEFAULT_SELECTED_COLOR)
        } else if !self.on_focus && !self.text_area.is_empty() {
            Style::default().fg(DEFAULT_SELECTED_COLOR)
        } else {
            Style::default().fg(DEFAULT_TEXT_COLOR)
        };

        if self.buffer.is_empty() && !self.on_focus {
            self.text_area = TextArea::from(vec!["Press <F> to find commands"])
        }

        if self.on_focus {
            self.text_area.set_cursor_line_style(Style::default());
            self.text_area.set_cursor_style(
                Style::default()
                    .fg(DEFAULT_TEXT_COLOR)
                    .add_modifier(Modifier::REVERSED),
            );
        } else {
            self.text_area.set_cursor_line_style(Style::default());
            self.text_area.set_cursor_style(Style::default());
        };

        self.text_area
            .set_block(Block::default().style(if !self.on_focus {
                Style::default().fg(DEFAULT_TEXT_COLOR)
            } else {
                Style::default()
            }));
        self.text_area.set_style(style);

        self.text_area.widget().render(area, buf)
    }
}
