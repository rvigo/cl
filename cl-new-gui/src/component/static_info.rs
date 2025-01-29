use crate::component::Renderable;
use tui::layout::Alignment::Center;
use tui::layout::Rect;
use tui::widgets::Paragraph;
use tui::Frame;

pub struct StaticInfo {
    pub content: String,
}

impl StaticInfo {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl Renderable for StaticInfo {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let paragraph = Paragraph::new(self.content.clone()).alignment(Center);

        frame.render_widget(paragraph, area);
    }
}
