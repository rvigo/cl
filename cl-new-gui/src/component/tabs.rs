use crate::component::Renderable;
use crate::screen::theme::Theme;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::Block;
use tui::Frame;
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tabs {
    items: Vec<String>,
    selected: usize,
    view_offset: usize,
}

impl Tabs {
    pub fn new() -> Tabs {
        Self {
            items: Vec::new(),
            selected: 0,
            view_offset: 0,
        }
    }

    pub fn select(&mut self, index: usize) {
        self.selected = index;
    }

    pub fn update_items(&mut self, items: Vec<String>) {
        self.items = items;
        self.view_offset = 0;
    }

    pub fn reset_selected(&mut self) {
        self.selected = 0;
        self.view_offset = 0;
    }

    /// Returns `(visible_items, adjusted_selected_index)`.
    ///
    /// - Truncates any name longer than `MAX_NAME_LEN` with `...`.
    /// - Keeps `view_offset` stable while `selected` is inside the current window.
    /// - Only re-centers (shifts `view_offset`) when `selected` goes out of view,
    ///   placing the selected tab in the middle of the new window.
    fn compute_visible_window(&mut self, area_width: u16) -> (Vec<String>, usize) {
        if self.items.is_empty() {
            return (vec![], 0);
        }

        // Account for block borders (left + right).
        let available = area_width.saturating_sub(2) as usize;

        const MAX_NAME_LEN: usize = 15;

        let names: Vec<String> = self.items.iter().map(|n| {
            if n.width() > MAX_NAME_LEN {
                let cut = n
                    .char_indices()
                    .nth(MAX_NAME_LEN - 3)
                    .map(|(i, _)| i)
                    .unwrap_or(n.len());
                format!("{}...", &n[..cut])
            } else {
                n.clone()
            }
        }).collect();

        // Each tab: 1 space padding left + name + 1 space padding right + 1 divider.
        let tab_widths: Vec<usize> = names.iter().map(|n| n.width() + 3).collect();
        let len = names.len();
        let sel = self.selected.min(len - 1);

        // If everything fits, no windowing needed.
        let total: usize = tab_widths.iter().sum();
        if total <= available {
            self.view_offset = 0;
            return (names, sel);
        }

        // Clamp view_offset in case items list shrank.
        if self.view_offset >= len {
            self.view_offset = 0;
        }

        // Count how many tabs are visible starting from the current view_offset.
        let visible_count = {
            let mut used = 0usize;
            let mut count = 0usize;
            for i in self.view_offset..len {
                if used + tab_widths[i] > available {
                    break;
                }
                used += tab_widths[i];
                count += 1;
            }
            count
        };
        let view_end = self.view_offset + visible_count;

        // If selected is outside the current window, re-center around it.
        if sel < self.view_offset || sel >= view_end {
            let mut start = sel;
            let mut end = sel;
            let mut used = tab_widths[sel];

            loop {
                let left_in_window = sel - start;
                let right_in_window = end - sel;
                let can_left = start > 0 && used + tab_widths[start - 1] <= available;
                let can_right = end < len - 1 && used + tab_widths[end + 1] <= available;

                if !can_left && !can_right {
                    break;
                }

                // Expand whichever side of the window (relative to selected) is shorter,
                // so the selected tab ends up roughly centered.
                if can_left && (left_in_window <= right_in_window || !can_right) {
                    start -= 1;
                    used += tab_widths[start];
                } else if can_right {
                    end += 1;
                    used += tab_widths[end];
                }
            }

            self.view_offset = start;
        }

        // Build the visible slice from the (possibly updated) view_offset.
        let mut visible_end = self.view_offset;
        let mut used = 0usize;
        while visible_end < len && used + tab_widths[visible_end] <= available {
            used += tab_widths[visible_end];
            visible_end += 1;
        }

        (names[self.view_offset..visible_end].to_vec(), sel - self.view_offset)
    }
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for Tabs {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let highlight_style = Style::default().fg(theme.highlight_color.into());

        let block_style = Style::default()
            .fg(theme.text_color.into())
            .bg(theme.background_color.into());

        let (visible_items, visible_selected) = self.compute_visible_window(area.width);

        let tabs = tui::widgets::Tabs::new(visible_items.clone())
            .divider("|")
            .highlight_style(highlight_style)
            .block(Block::bordered().style(block_style));

        let tabs = if !visible_items.is_empty() {
            tabs.select(visible_selected)
        } else {
            tabs
        };

        frame.render_widget(tabs, area);
    }
}
