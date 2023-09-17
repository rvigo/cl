use super::{display::DisplayWidget, Footer};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Widget},
};

#[derive(Clone)]
pub struct HelpFooter {
    content: String,
}

impl HelpFooter {
    pub fn new() -> HelpFooter {
        Self {
            content: String::from("Show help <F1/?>"),
        }
    }
}

impl Footer for HelpFooter {}

impl Widget for HelpFooter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .style(Style::default())
            .borders(Borders::ALL)
            .title(" Help ")
            .title_alignment(Alignment::Right)
            .border_type(BorderType::Plain);
        let display = DisplayWidget::new(self.content, true, false)
            .alignment(Alignment::Right)
            .block(block);

        display.render(area, buf)
    }
}
