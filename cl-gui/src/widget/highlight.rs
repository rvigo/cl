use super::display::DisplayWidget;
use itertools::Itertools;
use std::collections::HashSet;
use tui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

// Highlighting specific implementation
impl<'hl> DisplayWidget<'hl> {
    /// Highlights the `content` based on the given input
    ///
    /// If the input is empty, returns `Self` without any modification
    #[inline]
    pub fn highlight<T>(&mut self, highlight_input: T)
    where
        T: Into<String>,
    {
        let highlight_string: String = highlight_input.into();

        if !self.should_highlight() || highlight_string.is_empty() {
            return;
        }

        let content = self.raw_content();

        if highlight_string == content {
            let line = Line::styled(
                highlight_string,
                Style::default().add_modifier(Modifier::UNDERLINED),
            );
            self.update_content(line);
            return;
        }

        let splitted_content = self.split_preserve_chars(&content, &highlight_string);
        let grouped = self.group_by_newline(&splitted_content);

        let spans = grouped
            .into_iter()
            .flat_map(|line| {
                line.into_iter()
                    .map(|segment| {
                        let style = if self.contains_ignore_case(&highlight_string, &segment) {
                            Style::default().add_modifier(Modifier::UNDERLINED)
                        } else {
                            Style::default()
                        };

                        Span::styled(segment.to_owned(), style)
                    })
                    .collect_vec()
            })
            .collect_vec();

        self.update_content(Line::from(spans));
    }

    /// Splits the given content based on the given pattern
    ///
    /// # Example
    ///
    /// If the content is "this is a sandbox" and the pattern is "sand",
    ///
    /// the output should be `["thi", "s", " i", "s", " ", "a", " ", "s", "a", "n", "d", "box"]`
    #[inline]
    fn split_preserve_chars(&self, content: &'hl str, pattern: &'hl str) -> Vec<&'hl str> {
        let mut idxs = pattern
            .chars()
            .collect::<HashSet<_>>()
            .iter()
            .flat_map(|p| content.match_indices(*p).map(|(i, _)| i))
            .collect::<Vec<usize>>();

        // must be sorted
        idxs.sort();

        let mut result = Vec::new();
        let mut previous_index = 0;

        for index in idxs {
            if previous_index < index {
                result.push(&content[previous_index..index]);
            }

            let inclusive_char = &content[index..index + 1];
            result.push(inclusive_char);
            previous_index = index + 1;
        }

        if previous_index < content.len() {
            result.push(&content[previous_index..]);
        }

        result
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
    fn group_by_newline(&self, input: &'hl [&str]) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut sub_vec = Vec::new();

        for item in input {
            if let Some((before, after)) = item.split_once('\n') {
                sub_vec.push(before.to_string());
                result.push(sub_vec);
                sub_vec = Vec::new(); // here we finish one sub vec (representing an EOL) and starts a new empty sub vec

                if !after.is_empty() {
                    sub_vec.push(after.to_string());
                }
            } else {
                sub_vec.push(item.to_string());
            }
        }

        if !sub_vec.is_empty() {
            result.push(sub_vec);
        }

        result
    }

    #[inline]
    fn contains_ignore_case(&self, v1: &str, v2: &str) -> bool {
        v1.to_ascii_lowercase()
            .matches(&v2.to_ascii_lowercase())
            .next()
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::widget::{display::DisplayWidget, text_field::FieldType};

    #[test]
    fn should_group_highlighted_multilne_input() {
        let content = "multiline\ninput";
        let pattern = "in";
        let d = DisplayWidget::new(FieldType::Command, content, false, true, true);
        let input = d.split_preserve_chars(content, pattern);
        let expected = vec![vec!["mult", "i", "l", "i", "n", "e"], vec!["i", "n", "put"]];

        let result = d.group_by_newline(&input);

        assert_eq!(expected, result);
    }

    #[test]
    fn should_group_multilne_input_if_there_is_no_pattern() {
        let content = "multiline\ninput";
        let pattern = "";
        let d = DisplayWidget::new(FieldType::Command, content, false, true, true);
        let input = d.split_preserve_chars(content, pattern);
        let expected = vec![vec!["multiline"], vec!["input"]];

        let result = d.group_by_newline(&input);

        assert_eq!(expected, result);
    }
}
