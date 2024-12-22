use super::Component;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};

#[derive(PartialEq, Debug, Clone)]
pub struct DisplayWidget<'a> {
    /// the content of the widget
    pub content: String,
    /// flag indicating if the content can be highlighted
    pub should_highlight: bool,
    /// the highlighted content
    pub highlighted_content: Option<Line<'a>>,
    /// flag indicating if the content should be trimmed
    trim: bool,
    block: Option<Block<'a>>,
    style: Style,
    alignment: Alignment,
}

impl Component for DisplayWidget<'_> {}

impl<'a> DisplayWidget<'a> {
    pub fn new<T>(content: T, trim: bool, should_highlight: bool) -> DisplayWidget<'a>
    where
        T: Into<String>,
    {
        Self {
            content: content.into(),
            should_highlight,
            highlighted_content: None,
            trim,
            block: None,
            style: Default::default(),
            alignment: Alignment::Left,
        }
    }

    pub fn content(&self) -> String {
        self.content.to_owned()
    }

    pub fn set_highlighted_content(&mut self, highlight_content: Option<Line<'a>>) {
        self.highlighted_content = highlight_content
    }

    pub fn should_highlight(&self) -> bool {
        self.should_highlight
    }

    pub fn block(mut self, block: Block<'a>) -> DisplayWidget<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> DisplayWidget<'a> {
        self.style = style;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> DisplayWidget<'a> {
        self.alignment = alignment;
        self
    }
}

impl<'a> Widget for DisplayWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // if there is no highlighted content, transforms the content in a `Text`
        let content = if let Some(styled) = self.highlighted_content {
            Text::from(styled)
        } else {
            Text::from(self.content)
        };

        let paragraph = Paragraph::new(content)
            .style(self.style)
            .alignment(self.alignment)
            .wrap(Wrap { trim: self.trim })
            .block(if let Some(block) = self.block {
                block
            } else {
                Block::default()
            });

        paragraph.render(area, buf)
    }
}

#[cfg(test)]
mod test {
    use super::{DisplayWidget, *};
    use tui::{
        style::Modifier,
        text::{Line, Span},
    };

    #[test]
    fn should_highlight_multiline_input() {
        let content = "sandbox\nsandbox";
        let pattern = "sand";

        let result = DisplayWidget::new(content, false, true).highlight(pattern);

        assert!(result.highlighted_content.is_some());

        let expected_line = Line::from(vec![
            Span::styled("s", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::styled("a", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::styled("n", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::styled("d", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from("box"),
            Span::styled("s", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::styled("a", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::styled("n", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::styled("d", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from("box"),
        ]);

        let actual = result.highlighted_content.unwrap();
        assert_eq!(actual, expected_line);
    }

    #[test]
    fn should_highlight_single_line_input() {
        let content = "this is a sandbox";
        let pattern = "sand";

        let d = DisplayWidget::new(content, false, true);

        let expected_line = Line::from(vec![
            Span::from("thi"),
            Span::styled("s", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from(" i"),
            Span::styled("s", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from(" "),
            Span::styled("a", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from(" "),
            Span::styled("s", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::styled("a", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::styled("n", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::styled("d", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from("box"),
        ]);

        let result = d.highlight(pattern);

        assert!(result.highlighted_content.is_some());

        let actual = result.highlighted_content.unwrap();
        assert_eq!(actual, expected_line);
    }

    #[test]
    fn should_highlight_chars_in_content() {
        let content = "change location";
        let pattern = "cl";
        let d = DisplayWidget::new(content, false, true);

        let expected_line = Line::from(vec![
            Span::styled("c", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from("hange "),
            Span::styled("l", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from("o"),
            Span::styled("c", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from("ation"),
        ]);

        let result = d.highlight(pattern);

        assert!(result.highlighted_content.is_some());

        let actual = result.highlighted_content.unwrap();
        assert_eq!(actual, expected_line);
    }
}
