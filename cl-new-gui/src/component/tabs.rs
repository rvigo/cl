use crate::component::Renderable;
use crate::screen::theme::Theme;
use tui::layout::Rect;
use tui::style::Style;
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

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for Tabs {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let theme = theme.to_owned();
        let highlight_style = Style::default().fg(theme.highlight_color.into());

        let block_style = Style::default()
            .fg(theme.text_color.into())
            .bg(theme.background_color.into());
        let tabs = tui::widgets::Tabs::new(self.items.clone())
            .divider("|")
            .highlight_style(highlight_style)
            .block(Block::bordered().style(block_style));

        let tabs = if !self.items.is_empty() {
            tabs.select(self.selected)
        } else {
            tabs
        };

        frame.render_widget(tabs, area);
    }
}
