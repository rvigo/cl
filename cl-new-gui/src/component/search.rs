use crate::component::Renderable;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Clear, Paragraph};
use tui::Frame;
use tui_textarea::TextArea;

#[derive(Default, Debug)]
// TODO maybe change to quick search?
pub struct Search {
    pub textarea: TextArea<'static>,
}

impl Renderable for Search {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let content = self.textarea.lines().join("\n");
        let paragraph =
            Paragraph::new(content).block(Block::default().borders(Borders::ALL).title("Search"));
        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, area)
    }
}
