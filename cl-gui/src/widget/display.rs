use super::{text_field::FieldType, Component};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Paragraph, Widget, Wrap},
};

#[derive(PartialEq, Debug, Clone)]
pub struct DisplayWidget<'a> {
    /// the raw content of the widget
    raw_content: String,
    /// flag indicating if the content can be highlighted
    pub should_highlight: bool,
    /// the content
    pub content: Line<'a>,
    /// flag indicating if the content should be trimmed
    trim: bool,
    block: Option<Block<'a>>,
    style: Style,
    alignment: Alignment,
    pub r#type: FieldType,
    show_title: bool,
}

impl Component for DisplayWidget<'_> {}

impl<'a> DisplayWidget<'a> {
    pub fn new<T>(
        r#type: FieldType,
        content: T,
        trim: bool,
        should_highlight: bool,
        show_title: bool,
    ) -> DisplayWidget<'a>
    where
        T: Into<String>,
    {
        let content = content.into();
        Self {
            raw_content: content.to_owned(),
            should_highlight,
            content: Line::from(content),
            trim,
            block: None,
            style: Default::default(),
            alignment: Alignment::Left,
            r#type,
            show_title,
        }
    }

    pub fn raw_content(&self) -> String {
        self.raw_content.to_owned()
    }

    pub fn update_content<C>(&mut self, updated_content: C)
    where
        C: Into<Line<'a>>,
    {
        let updated_content: Line<'a> = updated_content.into();
        let raw = updated_content.spans.to_vec();

        let raw_content = raw
            .iter()
            .map(|span| span.content.clone())
            .collect::<String>();

        self.raw_content = raw_content;
        self.content = updated_content
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
        let block = self
            .block
            .map(|b| {
                if self.show_title {
                    b.title(self.r#type.to_string())
                } else {
                    b
                }
            })
            .unwrap_or_default();

        let content = self.content;
        let paragraph = Paragraph::new(content)
            .style(self.style)
            .alignment(self.alignment)
            .wrap(Wrap { trim: self.trim })
            .block(block);

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

        let mut widget = DisplayWidget::new(FieldType::Command, content, false, true, true);
        widget.highlight(pattern);

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

        let actual = widget.content;
        assert_eq!(actual, expected_line);
    }

    #[test]
    fn should_highlight_single_line_input() {
        let content = "this is a sandbox";
        let pattern = "sand";
        let mut widget = DisplayWidget::new(FieldType::Command, content, false, true, true);
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

        widget.highlight(pattern);

        let actual = widget.content;
        assert_eq!(actual, expected_line);
    }

    #[test]
    fn should_highlight_chars_in_content() {
        let content = "change location";
        let pattern = "cl";
        let mut widget = DisplayWidget::new(FieldType::Command, content, false, true, true);
        let expected_line = Line::from(vec![
            Span::styled("c", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from("hange "),
            Span::styled("l", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from("o"),
            Span::styled("c", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::from("ation"),
        ]);

        widget.highlight(pattern);

        let actual = widget.content;
        assert_eq!(actual, expected_line);
    }

    #[test]
    fn test() {
        let string = String::from("abc");
        let line = Line::from(string);

        let revert = line.spans.to_vec();

        let revert_line = &revert[0].content;

        println!("{:?}", revert_line);
    }
}
