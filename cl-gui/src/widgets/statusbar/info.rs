use super::StatusBarItem;
use crate::{widgets::display::DisplayWidget, DEFAULT_SELECTED_COLOR};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    widgets::Widget,
};

#[derive(Clone)]
pub struct Info {
    content: String,
}

impl Info {
    pub fn new<T: Into<String>>(content: T) -> Info {
        Self {
            content: content.into(),
        }
    }
}

impl Widget for Info {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let display = DisplayWidget::new(self.content, true, false)
            .alignment(Alignment::Center)
            .style(Style::default().bg(DEFAULT_SELECTED_COLOR));

        display.render(area, buf)
    }
}

impl StatusBarItem for Info {}
