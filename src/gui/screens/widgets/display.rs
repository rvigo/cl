use itertools::Itertools;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Spans, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};

#[derive(Clone)]
pub struct DisplayWidget<'a> {
    /// the content of the widget
    content: String,
    /// flag indicating if the content can be highlighted
    should_highlight: bool,
    /// the highlighted content
    highlighted_content: Option<Vec<Spans<'a>>>,
    /// flag indicating if the content should be trimmed
    trim: bool,
    block: Option<Block<'a>>,
    style: Style,
    alignment: Alignment,
}

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

    pub fn set_highlighted_content(&mut self, highlight_content: Option<Vec<Spans<'a>>>) {
        self.highlighted_content = highlight_content
    }

    pub fn should_highlight(&self) -> bool {
        self.should_highlight
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
    fn render(self, area: Rect, buf: &mut Buffer) {
        // if there is no highlighted content, transforms the content in a `Vec<Spans>`
        let content = if let Some(styled) = self.highlighted_content {
            Text::from(styled)
        } else {
            Text::from(self.content.split('\n').map(Spans::from).collect_vec())
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

#[cfg(test)]
mod test {
    use super::{DisplayWidget, *};
    use crate::gui::screens::widgets::highlight::Highlight;
    use tui::{style::Modifier, text::Span};

    #[test]
    fn should_highlight_multiline_input() {
        let content = "sandbox\nsandbox";
        let pattern = "sand";
        let d = DisplayWidget::new(content, false, true);

        let result = d.clone().highlight(pattern);

        assert!(result.highlighted_content.is_some());

        let expected_spans = vec![
            Spans::from(vec![
                Span::styled("s", Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::styled("a", Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::styled("n", Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::styled("d", Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::from("box"),
            ]),
            Spans::from(vec![
                Span::styled("s", Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::styled("a", Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::styled("n", Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::styled("d", Style::default().add_modifier(Modifier::UNDERLINED)),
                Span::from("box"),
            ]),
        ];

        let actual = result.highlighted_content.unwrap();
        assert_eq!(actual, expected_spans);
    }

    #[test]
    fn should_highlight_single_line_input() {
        let content = "this is a sandbox";
        let pattern = "sand";

        let d = DisplayWidget::new(content, false, true);

        let expected_spans = vec![Spans::from(vec![
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
        ])];

        let result = d.highlight(pattern);

        assert!(result.highlighted_content.is_some());

        let actual = result.highlighted_content.unwrap();
        assert_eq!(actual, expected_spans);
    }

    #[test]
    fn should_highlight_chars_in_content() {
        let content = "change location";
        let pattern = "cl";
        let d = DisplayWidget::new(content, false, true);
        let expected_spans = vec![Spans::from(vec![
            Span::styled("c", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from("hange "),
            Span::styled("l", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from("o"),
            Span::styled("c", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from("ation"),
        ])];

        let result = d.highlight(pattern);

        assert!(result.highlighted_content.is_some());

        let actual = result.highlighted_content.unwrap();
        assert_eq!(actual, expected_spans);
    }

    #[test]
    fn should_group_highlighted_multilne_input() {
        let content = "multiline\ninput";
        let pattern = "in";
        let d = DisplayWidget::new(content, false, true);
        let input = d.split_preserve_chars(content, pattern);
        let expected = vec![vec!["mult", "i", "l", "i", "n", "e"], vec!["i", "n", "put"]];

        let result = d.group_by_newline(&input);

        assert_eq!(expected, result);
    }

    #[test]
    fn should_group_multilne_input_if_there_is_no_pattern() {
        let content = "multiline\ninput";
        let pattern = "";
        let d = DisplayWidget::new(content, false, true);
        let input = d.split_preserve_chars(content, pattern);
        let expected = vec![vec!["multiline"], vec!["input"]];

        let result = d.group_by_newline(&input);

        assert_eq!(expected, result);
    }
}
