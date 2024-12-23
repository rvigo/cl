use crate::{default_popup_block, theme::DEFAULT_TEXT_COLOR, ViewMode};
use comfy_table::{presets, CellAlignment};
use std::ops::Deref;
use tui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Clear, Padding, Paragraph, Widget, Wrap},
};
use unicode_width::UnicodeWidthStr;

use super::popup_trait::PopupTrait;

pub struct HelpPopup<'help> {
    content: Table<'help>,
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

impl PopupTrait for HelpPopup<'_> {
    fn content_height(&self) -> u16 {
        self.content.len() as u16
    }

    fn content_width(&self) -> u16 {
        const FIXED_WIDTH: u16 = 75;

        FIXED_WIDTH
    }
}

impl<'p> Widget for HelpPopup<'p> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let render_position = self.get_render_position(area);
        let mut t = comfy_table::Table::new();
        t.load_preset(presets::NOTHING);
        for row in &self.content.content {
            t.add_row(row.cells.iter().map(|cell| cell.text).collect::<Vec<_>>());
        }
        t.column_iter_mut().for_each(|col| {
            col.set_constraint(comfy_table::ColumnConstraint::Absolute(
                comfy_table::Width::Fixed(self.content.width() + 7),
            ));
            col.set_cell_alignment(CellAlignment::Left);
        });

        let p = Paragraph::new(t.to_string())
            .style(Style::default().fg(DEFAULT_TEXT_COLOR))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(default_popup_block!(Type::Help).padding(Padding::vertical(2)));

        Clear::render(Clear, render_position, buf);
        p.render(render_position, buf)
    }
}

macro_rules! cell {
    ($text:expr) => {
        Cell::from($text)
    };
}

macro_rules! row {
    ($( $cell:expr),+ $(,)?) => {{
        let mut row = Row::default();
        $(
            row.add_cell($cell);
        )*

        row
        }};

}
macro_rules! table{
    ($( $row:expr),+ $(,)?) => {{
        let mut table= vec![];
        $(
           table.push($row);
        )*

       table.into()
        }};

}

fn main_options<'a>() -> Table<'a> {
    table! {
        row! {cell!("Quit"), cell!("<Q/Esc/Ctrl-C>")},
        row! {cell!("Create new command"), cell!("<I/Insert>")},
        row! {cell!("Delete selected command"), cell!("<D/Delete>")},
        row! {cell!("Edit selected command"), cell!("<E>")},
        row! {cell!("Move to next namespace"), cell!("<L/→/Tab>")},
        row! {cell!("Move to previous namespace"), cell!("<H/←/Shift-Tab>")},
        row! {cell!("Move up"), cell!("<K/↑>")},
        row! {cell!("Move down"), cell!("<J/↓>")},
        row! {cell!("Copy selected command"), cell!("<Y>")},
        row! {cell!("Search commands"), cell!("<F//>")},
        row! {cell!("Show help"), cell!("<F1/?>")},
    }
}

fn edit_options<'a>() -> Table<'a> {
    table! {
            row! { cell!("Return"), cell!("<Esc/Ctrl-C>")},
            row! { cell!("Next Field"), cell!("<Tab>")},
            row! { cell!("Previous Field"), cell!("<Shift-Tab>")},
            row! { cell!("Update command"), cell!("<Enter/Ctrl-S>")},
            row! { cell!("Help"), cell!("<F1>")},
    }
}

fn insert_options<'a>() -> Table<'a> {
    table! {
            row! { cell!("Return"), cell!("<Esc/Ctrl-C>")},
            row! { cell!("Next Field"), cell!("<Tab>")},
            row! { cell!("Previous Field"), cell!("<Shift-Tab>")},
            row! { cell!("Create command"), cell!("<Enter/Ctrl-S>")},
            row! { cell!("Help"), cell!("<F1>")},
    }
}

#[derive(Clone)]
struct Table<'a> {
    content: Vec<Row<'a>>,
}

impl Table<'_> {
    fn width(&self) -> u16 {
        self.content
            .iter()
            .map(|row| row.width())
            .max()
            .unwrap_or(0)
    }
}

impl<'a> From<Vec<Row<'a>>> for Table<'a> {
    fn from(content: Vec<Row<'a>>) -> Self {
        Table { content }
    }
}

impl<'a> Deref for Table<'a> {
    type Target = Vec<Row<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

#[derive(Clone, Default)]
pub struct Row<'a> {
    cells: Vec<Cell<'a>>,
}

impl<'a> Row<'a> {
    pub fn add_cell(&mut self, cell: Cell<'a>) {
        self.cells.push(cell);
    }

    pub fn width(&self) -> u16 {
        self.cells
            .iter()
            .map(|cell| cell.width())
            .max()
            .unwrap_or(0)
    }
}

#[derive(Clone)]
pub struct Cell<'a> {
    text: &'a str,
}

impl<'a> Cell<'a> {
    fn new(text: &'a str) -> Cell<'a> {
        Cell { text }
    }

    fn width(&self) -> u16 {
        self.text.width() as u16
    }
}

impl<'a> From<&'a str> for Cell<'a> {
    fn from(text: &'a str) -> Self {
        Cell::new(text)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_main_options() {
        let cell1 = Cell::from("Quit");
        let cell2 = Cell::from("Move to previous namespace");

        let current = vec![&cell1, &cell2]
            .iter()
            .map(|cell| cell.width())
            .max()
            .unwrap_or(0);
        let cell1_width = cell1.text.width();
        let cell2_width = cell2.text.width();
        let r = row! {
            cell1,
            cell2,
        };

        let row_width = r.width();
        assert_eq!(cell1_width, 4);
        assert_eq!(cell2_width, 26);
        assert_eq!(current, 26);
        assert_eq!(row_width, 26);
    }
}
