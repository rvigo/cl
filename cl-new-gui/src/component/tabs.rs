use crate::component::Renderable;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::Block;
use tui::Frame;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tabs {
    items: Vec<String>,
    selected: usize,
}

impl Tabs {
    pub fn new() -> Tabs {
        Self {
            items: Vec::new(),
            selected: 0,
        }
    }

    pub fn next(&mut self, next: usize) {
        self.selected = next
    }

    pub fn previous(&mut self, previous: usize) {
        self.selected = previous
    }

    pub fn update_items(&mut self, items: Vec<String>) {
        self.items = items;
    }

    pub fn reset_selected(&mut self) {
        self.selected = 0
    }
}

impl Renderable for Tabs {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let tabs = tui::widgets::Tabs::new(self.items.clone())
            .select(self.selected)
            .divider("|")
            .highlight_style(Style::default().fg(Color::Yellow))
            .block(Block::bordered());

        frame.render_widget(tabs, area);
    }
}
