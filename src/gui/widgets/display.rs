use crate::gui::layouts::DEFAULT_SELECTED_COLOR;
use itertools::Itertools;
use tui::{
    layout::Alignment,
    style::Style,
    text::{Span, Spans, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};

#[derive(Clone)]
pub struct DisplayWidget<'a> {
    content: String,
    highlighted_content: Option<Spans<'a>>,
    trim: bool,
    title: String,
    block: Option<Block<'a>>,
    style: Style,
    alignment: Alignment,
}

impl<'a> DisplayWidget<'a> {
    pub fn new<T>(content: T, trim: bool) -> DisplayWidget<'a>
    where
        T: Into<String>,
    {
        Self {
            content: content.into(),
            highlighted_content: None,
            trim,
            title: String::default(),
            block: None,
            style: Default::default(),
            alignment: Alignment::Left,
        }
    }

    pub fn title<T>(mut self, title: T) -> DisplayWidget<'a>
    where
        T: Into<String>,
    {
        self.title = title.into();
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

    pub fn highlight(mut self, highlight_string: String) -> DisplayWidget<'a> {
        if highlight_string.eq(&self.content) {
            let span = Span::styled(
                highlight_string.to_owned(),
                Style::default().bg(DEFAULT_SELECTED_COLOR),
            );
            self.highlighted_content = Some(Spans::from(span));
            return self;
        }

        let splitted_content = self.split_preserve_chars(&self.content, &highlight_string);
        let spans = splitted_content
            .iter()
            .map(|c| {
                let style = if self.contains_ignore_case(&highlight_string, c) {
                    Style::default().bg(DEFAULT_SELECTED_COLOR)
                } else {
                    Style::default()
                };

                Span::styled(String::from(c.to_owned()), style)
            })
            .collect_vec();

        self.highlighted_content = Some(Spans::from(spans));

        self
    }

    fn split_preserve_chars(&self, content: &'a str, pattern: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();
        let mut last_end = 0;

        for (i, c) in pattern.chars().enumerate() {
            while let Some(start) = content[last_end..].find(c) {
                let end = last_end + start;
                result.push(&content[last_end..end]);
                result.push(&content[end..end + c.len_utf8()]);
                last_end = end + c.len_utf8();
            }

            if i < pattern.len() - 1 && last_end < content.len() {
                result.push(&content[last_end..last_end + 1]);
                last_end += 1;
            }
        }

        if last_end < content.len() {
            result.push(&content[last_end..]);
        }

        result
    }

    fn contains_ignore_case(&self, v1: &str, v2: &str) -> bool {
        v1.to_lowercase().contains(&v2.to_lowercase())
    }
}

impl<'a> Widget for DisplayWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        let content = if let Some(styled) = self.highlighted_content {
            Text::from(styled)
        } else {
            Text::from(self.content)
        };

        let p = Paragraph::new(content)
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
