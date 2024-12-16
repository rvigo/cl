use super::Component;
use tui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Widget},
};

#[derive(Clone)]
pub struct Tabs<'tabs> {
    pub titles: Vec<String>,
    pub selected: usize,
    pub divider: char,
    pub style: Style,
    pub block: Option<Block<'tabs>>,
    pub highlight_style: Style,
}

impl Component for Tabs<'_> {}

impl<'tabs> Tabs<'tabs> {
    pub fn new(titles: Vec<String>) -> Self {
        Self {
            titles,
            selected: 0,
            divider: '|',
            style: Style::default(),
            block: None,
            highlight_style: Style::default(),
        }
    }

    pub fn select(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }

    pub fn divider(mut self, divider: char) -> Self {
        self.divider = divider;
        self
    }

    pub fn block(mut self, block: Block<'tabs>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }
}

impl Widget for Tabs<'_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let tabs_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if tabs_area.height < 1 {
            return;
        }

        let mut x = tabs_area.left();
        let titles_length = self.titles.len();
        for (i, title) in self.titles.into_iter().enumerate() {
            let last_title = titles_length - 1 == i;
            x = x.saturating_add(1);
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 {
                break;
            }
            let pos = buf.set_line(x, tabs_area.top(), &Line::from(title), remaining_width);
            if i == self.selected {
                buf.set_style(
                    Rect {
                        x,
                        y: tabs_area.top(),
                        width: pos.0.saturating_sub(x),
                        height: 1,
                    },
                    self.highlight_style,
                );
            }
            x = pos.0.saturating_add(1);
            let remaining_width = tabs_area.right().saturating_sub(x);
            if remaining_width == 0 || last_title {
                break;
            }
            let pos = buf.set_line(
                x,
                tabs_area.top(),
                &Line::from(self.divider.to_string()),
                remaining_width,
            );
            x = pos.0;
        }
    }
}
