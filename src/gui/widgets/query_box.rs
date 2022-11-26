use crate::gui::layouts::{DEFAULT_SELECTED_COLOR, DEFAULT_TEXT_COLOR};
use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Widget},
};
use tui_textarea::{CursorMove, TextArea};

#[derive(Clone)]
pub struct QueryBox<'a> {
    text_area: TextArea<'a>,
    title: String,
    on_focus: bool,
    buffer: String,
}

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
    pub fn handle(&mut self, key_event: KeyEvent) {
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

    pub fn toggle_focus(&mut self) {
        self.on_focus = !self.on_focus
    }

    pub fn is_on_focus(&self) -> bool {
        self.on_focus
    }

    pub fn get_input(&self) -> String {
        self.buffer.clone()
    }
}

impl<'a> Widget for QueryBox<'a> {
    fn render(mut self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let style = if self.on_focus {
            Style::default().fg(Color::Black).bg(DEFAULT_SELECTED_COLOR)
        } else if !self.on_focus && !self.text_area.is_empty() {
            Style::default().fg(DEFAULT_SELECTED_COLOR)
        } else {
            Style::default().fg(DEFAULT_TEXT_COLOR)
        };

        if self.buffer.is_empty() && !self.on_focus {
            self.text_area = TextArea::from(vec!["Press <F> to find commands"])
        } else {
            self.text_area = TextArea::from(vec![self.buffer])
        }

        if self.on_focus {
            self.text_area.set_cursor_line_style(Style::default());
            self.text_area
                .set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
            self.text_area.move_cursor(CursorMove::End);
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
        self.text_area.set_alignment(tui::layout::Alignment::Left);
        self.text_area.set_style(style);

        self.text_area.widget().render(area, buf)
    }
}
