use tui::Frame;
use tui::layout::Rect;
use tui::widgets::Paragraph;
use crate::component::Renderable;

pub struct StaticInfo {
    pub content: String
}

impl StaticInfo {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into()
        }
    }
}

impl Renderable for StaticInfo {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let paragraph = Paragraph::new(self.content.clone());
        
        frame.render_widget(paragraph, area);
    }
}