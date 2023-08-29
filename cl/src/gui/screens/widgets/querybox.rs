use super::{Footer, WidgetKeyHandler};
use crate::gui::{DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR};
use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Widget},
};
use tui_textarea::TextArea;

#[derive(Clone)]
pub struct QueryBox<'a> {
    text_area: TextArea<'a>,
    title: String,
    on_focus: bool,
    buffer: String,
}

impl<'a> Footer for QueryBox<'a> {}

impl<'a> Default for QueryBox<'a> {
    fn default() -> Self {
        Self {
            text_area: TextArea::default(),
            title: String::from("Find"),
            on_focus: false,
            buffer: String::default(),
        }
    }
}

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

        self.text_area.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(if !self.on_focus {
                    Style::default().fg(DEFAULT_TEXT_COLOR)
                } else {
                    Style::default()
                })
                .title(format!(" {} ", self.title))
                .border_type(BorderType::Plain),
        );
        self.text_area.set_alignment(Alignment::Left);
        self.text_area.set_style(style);

        self.text_area.widget().render(area, buf)
    }
}
