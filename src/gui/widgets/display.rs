use tui::{
    layout::Alignment,
    style::Style,
    text::Text,
    widgets::{Block, Paragraph, Widget, Wrap},
};

pub struct DisplayWidget<'a> {
    content: Text<'a>,
    trim: bool,
    title: String,
    block: Option<Block<'a>>,
    style: Style,
    alignment: Alignment,
}

impl<'a> DisplayWidget<'a> {
    pub fn new<T>(content: T, trim: bool) -> DisplayWidget<'a>
    where
        T: Into<Text<'a>>,
    {
        Self {
            content: content.into(),
            trim,
            title: String::default(),
            block: None,
            style: Default::default(),
            alignment: Alignment::Left,
        }
    }

    pub fn title(mut self, title: String) -> DisplayWidget<'a> {
        self.title = title;
        self
    }

    pub fn block(mut self, block: Block<'a>) -> DisplayWidget<'a> {
        self.block = Some(block);
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> DisplayWidget<'a> {
        self.alignment = alignment;
        self
    }
}

impl<'a> Widget for DisplayWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let p = Paragraph::new(self.content)
            .style(self.style)
            .alignment(self.alignment)
            .wrap(Wrap { trim: self.trim })
            .block(if let Some(block) = self.block {
                block
            } else {
                Block::default()
            });

        p.render(area, buf)
    }
}
