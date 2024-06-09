use crate::{dummy_block, widget::display::DisplayWidget};
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
            content: String::from("Help <F1/?>"),
        }
    }
}

impl Widget for Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let inner_b = dummy_block!();
        let inner_area = inner_b.inner(area);

        let display = DisplayWidget::new(self.content, true, false).alignment(Alignment::Right);

        display.render(inner_area, buf)
    }
}

impl Default for Help {
    fn default() -> Self {
        Self::new()
    }
}
