use itertools::Itertools;
use tui::{
    layout::Alignment,
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Paragraph, Widget, Wrap},
};

#[derive(Clone)]
pub struct DisplayWidget<'a> {
    content: String,
    highlighted_content: Option<Vec<Spans<'a>>>,
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

    /// Highlights the `content` based on the given input
    ///
    /// If the input is empty, returns `Self` without any modification
    #[inline]
    pub fn highlight<T>(mut self, highlight_string: T) -> DisplayWidget<'a>
    where
        T: Into<String>,
    {
        let highlight_string: String = highlight_string.into();
        if highlight_string.is_empty() {
            return self;
        }

        if highlight_string.eq(&self.content) {
            let span = Span::styled(
                highlight_string,
                Style::default().add_modifier(Modifier::UNDERLINED),
            );
            self.highlighted_content = Some(vec![Spans::from(span)]);
            return self;
        }

        let splitted_content = self.split_preserve_chars(&self.content, &highlight_string);
        let grouped = self.group_by_newline(&splitted_content);

        let spans = grouped
            .into_iter()
            .map(|sv| {
                sv.iter()
                    .map(|i| {
                        let style = if self.contains_ignore_case(&highlight_string, i) {
                            Style::default().add_modifier(Modifier::UNDERLINED)
                        } else {
                            Style::default()
                        };

                        Span::styled(i.to_owned(), style)
                    })
                    .collect_vec()
            })
            .map(Spans::from)
            .collect_vec();

        self.highlighted_content = Some(spans);

        self
    }

    /// Splits the given content based on the given pattern
    ///
    /// # Example
    ///
    /// If the content is "this is a sandbox" and the pattern is "sand",
    ///
    /// the output should be `["thi", "s", " i", "s", " ", "a", " ", "s", "a", "n", "d", "box"]`
    #[inline]
    fn split_preserve_chars(&self, content: &'a str, pattern: &'a str) -> Vec<&'a str> {
        let mut idxs = Vec::default();

        for p in pattern.chars().unique() {
            let matches = content.match_indices(p);
            idxs.extend(matches.map(|(i, _)| i));
        }

        // must be sorted
        idxs.sort();

        let mut result = Vec::new();
        let mut previous_index = 0;
        for index in idxs {
            let slice_untill_idx = &content[previous_index..index];
            result.push(slice_untill_idx);
            let inclusive_char = &content[index..index + 1];
            result.push(inclusive_char);
            previous_index = index + 1;
        }
        result.push(&content[previous_index..]);
        result.into_iter().filter(|s| !s.is_empty()).collect()
    }

    /// Groups the content of a `Vec<&str>` in a `Vec<Vec<String>>`
    ///
    /// If any item contains a `\n` char, a new inner Vec<String> will be created
    /// # Example
    ///
    /// If the content is `["a single line input"]`, the output should be `[["a single line input"]]`
    ///
    /// If the content is  `["multiline\n", "input"]`, the output should be `[["multiline"], ["input"]]`
    #[inline]
    fn group_by_newline(&self, input: &'a Vec<&str>) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut sub_vec = Vec::new();

        for item in input {
            if item.contains('\n') {
                let sub_items: Vec<&str> = item.split('\n').into_iter().collect();
                sub_vec.push(sub_items[0]);
                result.push(sub_vec);
                sub_vec = Vec::new(); // here we finish one sub vec (representing an EOL) and starts a new empty sub vec
                if sub_items.len() > 1 && !sub_items[1].is_empty() {
                    sub_vec.push(sub_items[1]);
                }
            } else {
                sub_vec.push(item);
            }
        }

        if !sub_vec.is_empty() {
            result.push(sub_vec);
        }

        result
            .into_iter()
            .filter(|sub_vec| !sub_vec.is_empty())
            .map(|sub_vec| sub_vec.iter().cloned().map(String::from).collect_vec())
            .collect()
    }

    fn contains_ignore_case(&self, v1: &str, v2: &str) -> bool {
        v1.to_ascii_lowercase()
            .matches(&v2.to_ascii_lowercase())
            .next()
            .is_some()
    }
}

impl<'a> Widget for DisplayWidget<'a> {
    fn render(self, area: tui::layout::Rect, buf: &mut tui::buffer::Buffer) {
        // if there is no highlighted content, transforms the content in a `Vec<Spans>`
        let content = if let Some(styled) = self.highlighted_content {
            Text::from(styled)
        } else {
            Text::from(
                self.content
                    .split('\n')
                    .into_iter()
                    .map(Spans::from)
                    .collect_vec(),
            )
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
    use super::DisplayWidget;
    use super::*;

    #[test]
    fn should_highlight_multiline_input() {
        let content = "sandbox\nsandbox";
        let pattern = "sand";
        let d = DisplayWidget::new(content, false);

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

        let d = DisplayWidget::new(content, false);

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
        let d = DisplayWidget::new(content, false);
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
        let d = DisplayWidget::new(content, false);
        let input = d.split_preserve_chars(content, pattern);
        let expected = vec![vec!["mult", "i", "l", "i", "n", "e"], vec!["i", "n", "put"]];

        let result = d.group_by_newline(&input);

        assert_eq!(expected, result);
    }

    #[test]
    fn should_group_multilne_input_if_there_is_no_pattern() {
        let content = "multiline\ninput";
        let pattern = "";
        let d = DisplayWidget::new(content, false);
        let input = d.split_preserve_chars(content, pattern);
        let expected = vec![vec!["multiline"], vec!["input"]];

        let result = d.group_by_newline(&input);

        assert_eq!(expected, result);
    }
}
