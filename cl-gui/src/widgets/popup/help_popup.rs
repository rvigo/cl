use super::{popup_type::PopupType, Popup};
use crate::{
    entities::{contexts::popup_context::PopupContext, view_mode::ViewMode},
    DEFAULT_SELECTED_COLOR,
};
use tui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Cell, Clear, Row, Table, Widget},
};

pub struct HelpPopup<'a> {
    content: Vec<Pair<Cell<'a>>>,
}

impl<'a> HelpPopup<'a> {
    pub fn new(view_mode: &ViewMode) -> HelpPopup<'a> {
        let content = match view_mode {
            ViewMode::Main => main_options(),
            ViewMode::Edit => edit_options(),
            ViewMode::Insert => insert_options(),
        };
        HelpPopup { content }
    }
}

impl Popup for HelpPopup<'_> {
    fn content_height(&self) -> u16 {
        self.content.len() as u16
    }

    fn content_width(&self) -> u16 {
        const FIXED_WIDTH: u16 = 75;

        FIXED_WIDTH
    }

    fn render(self, area: Rect, buf: &mut Buffer, _: Option<&mut PopupContext>) {
        let rows = self
            .content
            .clone()
            .into_iter()
            .map(|cells| Row::new(cells).bottom_margin(1));

        let table = Table::new(rows)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" {} ", PopupType::Help.to_string()))
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Plain),
            )
            .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

        let render_position = self.get_render_position(area);

        Clear::render(Clear, render_position, buf);
        table.render(render_position, buf)
    }
}

fn key_style() -> Style {
    Style::default().fg(DEFAULT_SELECTED_COLOR)
}

macro_rules! styled_cell {
    ($text:expr) => {
        Cell::from($text)
    };

    ($text:expr, $style:expr) => {
        Cell::from($text).style($style)
    };
}

fn main_options<'a>() -> Vec<Pair<Cell<'a>>> {
    vec![
        Pair::new(
            styled_cell!("<Q/Esc/Ctrl + C>", key_style()),
            styled_cell!("Quit"),
        ),
        Pair::new(
            styled_cell!("<I/Insert>", key_style()),
            styled_cell!("Create new command"),
        ),
        Pair::new(
            styled_cell!("<D/Delete>", key_style()),
            styled_cell!("Delete selected command"),
        ),
        Pair::new(
            styled_cell!("<E>", key_style()),
            styled_cell!("Edit selected command"),
        ),
        Pair::new(
            styled_cell!("<L/→/Tab>", key_style()),
            styled_cell!("Move to next namespace"),
        ),
        Pair::new(
            styled_cell!("<H/←/Shift + Tab>", key_style()),
            styled_cell!("Move to previous namespace"),
        ),
        Pair::new(styled_cell!("<K/↑>", key_style()), styled_cell!("Move up")),
        Pair::new(
            styled_cell!("<J/↓>", key_style()),
            styled_cell!("Move down"),
        ),
        Pair::new(
            styled_cell!("<Y>", key_style()),
            styled_cell!("Copy selected command"),
        ),
        Pair::new(
            styled_cell!("<F//>", key_style()),
            styled_cell!("Find stored commands"),
        ),
        Pair::new(
            styled_cell!("<F1/?>", key_style()),
            styled_cell!("Show help"),
        ),
    ]
}

fn edit_options<'a>() -> Vec<Pair<Cell<'a>>> {
    vec![
        Pair::new(
            styled_cell!("<Esc/Ctrl + C>", key_style()),
            styled_cell!("Return"),
        ),
        Pair::new(
            styled_cell!("<Tab>", key_style()),
            styled_cell!("Next Field"),
        ),
        Pair::new(
            styled_cell!("<Shift + Tab>", key_style()),
            styled_cell!("Previous Field"),
        ),
        Pair::new(
            styled_cell!("<Enter/ Ctrl + S>", key_style()),
            styled_cell!("Update command"),
        ),
        Pair::new(styled_cell!("<F1>", key_style()), styled_cell!("Help")),
    ]
}

fn insert_options<'a>() -> Vec<Pair<Cell<'a>>> {
    vec![
        Pair::new(
            styled_cell!("<Esc/Ctrl + C>", key_style()),
            styled_cell!("Return"),
        ),
        Pair::new(
            styled_cell!("<Tab>", key_style()),
            styled_cell!("Next Field"),
        ),
        Pair::new(
            styled_cell!("<Shift + Tab>", key_style()),
            styled_cell!("Previous Field"),
        ),
        Pair::new(
            styled_cell!("<Enter/ Ctrl + S>", key_style()),
            styled_cell!("Create command"),
        ),
        Pair::new(styled_cell!("<F1>", key_style()), styled_cell!("Help")),
    ]
}

#[derive(Clone)]
pub struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    pub fn new(first: T, second: T) -> Pair<T> {
        Self { first, second }
    }
}

impl<T> Iterator for PairIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(self.pair.first.to_owned()),
            1 => Some(self.pair.second.to_owned()),
            _ => None,
        };
        self.index += 1;
        result
    }
}

impl<T> IntoIterator for Pair<T>
where
    T: Clone,
{
    type Item = T;

    type IntoIter = PairIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        PairIterator {
            pair: self,
            index: 0,
        }
    }
}

pub struct PairIterator<T> {
    pair: Pair<T>,
    index: usize,
}
