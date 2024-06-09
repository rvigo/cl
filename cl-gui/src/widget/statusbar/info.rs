use crate::{dummy_block, widget::display::DisplayWidget, DEFAULT_INFO_COLOR};
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
        let inner_block = dummy_block!();
        let inner_area = inner_block.inner(area);

        let display = DisplayWidget::new(self.content, true, false)
            .alignment(Alignment::Center)
            .style(Style::default().bg(DEFAULT_INFO_COLOR));

        display.render(inner_area, buf)
    }
}
