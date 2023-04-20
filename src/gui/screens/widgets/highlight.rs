use super::display::DisplayWidget;
use itertools::Itertools;
use tui::{
    style::{Modifier, Style},
    text::{Span, Spans},
};

pub trait Highlight<'a> {
    fn highlight<T>(self, highlight_input: T) -> Self
    where
        T: Into<String>;

    fn split_preserve_chars(&self, content: &'a str, pattern: &'a str) -> Vec<&'a str>; // TODO is this the correct place for these aux methods?

    fn group_by_newline(&self, input: &'a [&str]) -> Vec<Vec<String>>;

    #[inline]
    fn contains_ignore_case(&self, v1: &str, v2: &str) -> bool {
        v1.to_ascii_lowercase()
            .matches(&v2.to_ascii_lowercase())
            .next()
            .is_some()
    }
}

impl<'a> Highlight<'a> for DisplayWidget<'a> {
    /// Highlights the `content` based on the given input
    ///
    /// If the input is empty, returns `Self` without any modification
    #[inline]
    fn highlight<T>(mut self, highlight_input: T) -> Self
    where
        T: Into<String>,
    {
        let highlight_string: String = highlight_input.into();
        if !self.should_highlight() || highlight_string.is_empty() {
            return self;
        }

        if highlight_string.eq(&self.content()) {
            let span = Span::styled(
                highlight_string,
                Style::default().add_modifier(Modifier::UNDERLINED),
            );
            self.set_highlighted_content(Some(vec![Spans::from(span)]));
            return self;
        }

        let content = self.content();
        let splitted_content = self.split_preserve_chars(&content, &highlight_string);
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

        self.set_highlighted_content(Some(spans));

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
    fn group_by_newline(&self, input: &'a [&str]) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut sub_vec = Vec::new();

        for item in input {
            if item.contains('\n') {
                let sub_items: Vec<&str> = item.split('\n').collect();
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
}
