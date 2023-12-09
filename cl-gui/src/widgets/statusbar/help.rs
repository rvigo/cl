use super::StatusBarItem;
use crate::widgets::display::DisplayWidget;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    widgets::Widget,
};

#[derive(Clone)]
pub struct Help {
    content: String,
}

impl Help {
    pub fn new() -> Help {
        Self {
            content: String::from("Show help <F1/?>"),
        }
    }
}

impl StatusBarItem for Help {}

impl Widget for Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let display = DisplayWidget::new(self.content, true, false).alignment(Alignment::Right);

        display.render(area, buf)
    }
}

impl Default for Help {
    fn default() -> Self {
        Self::new()
    }
}
