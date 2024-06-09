use crate::{default_widget_block, state::ListState, DEFAULT_HIGH_LIGHT_COLOR};
use cl_core::CommandVec;
use tui::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::{Color, Modifier, Style, Styled},
    text::Line,
    widgets::{Block, HighlightSpacing, Widget},
};
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct List<'a> {
    block: Option<Block<'a>>,
    items: Vec<String>,
    style: Style,
    start_corner: Corner,
    highlight_style: Style,
    highlight_symbol: Option<&'a str>,
    repeat_highlight_symbol: bool,
    highlight_spacing: HighlightSpacing,
    state: ListState,
}

impl<'a> List<'a> {
    pub fn new(commands: &CommandVec, state: ListState) -> List<'a> {
        let items: Vec<String> = commands.iter().map(|c| c.alias.to_owned()).collect();

        List {
            block: None,
            style: Style::default(),
            items,
            start_corner: Corner::TopLeft,
            highlight_style: Style::default()
                .fg(Color::Black)
                .bg(DEFAULT_HIGH_LIGHT_COLOR)
                .add_modifier(Modifier::BOLD | Modifier::ITALIC),

            highlight_symbol: Some("> "),
            repeat_highlight_symbol: false,
            highlight_spacing: HighlightSpacing::default(),
            state,
        }
    }

    pub fn style(mut self, style: Style) -> List<'a> {
        self.style = style;
        self
    }

    fn get_items_bounds(
        &self,
        selected: Option<usize>,
        offset: usize,
        max_height: usize,
    ) -> (usize, usize) {
        let offset = offset.min(self.items.len().saturating_sub(1));
        let mut start = offset;
        let mut end = offset;
        let mut height = 0;
        for item in self.items.iter().skip(offset) {
            if height + item.height() > max_height {
                break;
            }
            height += item.height();
            end += 1;
        }

        let selected = selected.unwrap_or(0).min(self.items.len() - 1);
        while selected >= end {
            height = height.saturating_add(self.items[end].height());
            end += 1;
            while height > max_height {
                height = height.saturating_sub(self.items[start].height());
                start += 1;
            }
        }
        while selected < start {
            start -= 1;
            height = height.saturating_add(self.items[start].height());
            while height > max_height {
                end -= 1;
                height = height.saturating_sub(self.items[end].height());
            }
        }
        (start, end)
    }
}

impl<'a> Widget for List<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => {
                let b = default_widget_block!("Aliases");
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        if self.items.is_empty() {
            return;
        }
        let list_height = list_area.height as usize;

        let (start, end) =
            self.get_items_bounds(self.state.selected, self.state.offset, list_height);
        self.state.offset = start;

        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = " ".repeat(highlight_symbol.width());

        let mut current_height = 0;
        let selection_spacing = self
            .highlight_spacing
            .should_add(self.state.selected.is_some());
        for (i, item) in self
            .items
            .iter_mut()
            .enumerate()
            .skip(self.state.offset)
            .take(end - start)
        {
            let (x, y) = if self.start_corner == Corner::BottomLeft {
                current_height += 1;
                (list_area.left(), list_area.bottom() - current_height)
            } else {
                let pos = (list_area.left(), list_area.top() + current_height);
                current_height += item.height() as u16;
                pos
            };
            let area = Rect {
                x,
                y,
                width: list_area.width,
                height: item.height() as u16,
            };
            let item_style = self.style;
            buf.set_style(area, item_style);

            let is_selected = self.state.selected.map_or(false, |s| s == i);
            for (j, line) in item.lines().enumerate() {
                // if the item is selected, we need to display the highlight symbol:
                // - either for the first line of the item only,
                // - or for each line of the item if the appropriate option is set
                let symbol = if is_selected && (j == 0 || self.repeat_highlight_symbol) {
                    highlight_symbol
                } else {
                    &blank_symbol
                };
                let (elem_x, max_element_width) = if selection_spacing {
                    let (elem_x, _) = buf.set_stringn(
                        x,
                        y + j as u16,
                        symbol,
                        list_area.width as usize,
                        item_style,
                    );
                    (elem_x, (list_area.width - (elem_x - x)))
                } else {
                    (x, list_area.width)
                };
                buf.set_line(elem_x, y + j as u16, &Line::from(line), max_element_width);
            }
            if is_selected {
                buf.set_style(area, self.highlight_style);
            }
        }
    }
}

impl<'a> Styled for List<'a> {
    type Item = List<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}

trait StringExt {
    fn height(&self) -> usize;
}

impl StringExt for String {
    fn height(&self) -> usize {
        self.lines().count()
    }
}
