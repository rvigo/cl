use crate::{
    event::PopupCallbackAction,
    widget::popup::{Choice, Popup, Type},
    ViewMode,
};
use std::slice::Iter;
use std::{
    fmt::{self},
    ops::Deref,
};
use unicode_width::UnicodeWidthStr;

pub struct HelpPopup;

impl HelpPopup {
    pub fn create(view_mode: &ViewMode) -> Popup {
        let content = match view_mode {
            ViewMode::Main => main_options(),
            ViewMode::Edit => edit_options(),
            ViewMode::Insert => insert_options(),
        };
        let choices = Choice::empty();

        Popup::new(
            content.as_vec(),
            choices,
            Type::Help,
            PopupCallbackAction::None,
        )
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

fn main_options() -> Table {
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

fn edit_options() -> Table {
    table! {
            row! { cell!("Return"), cell!("<Esc/Ctrl-C>")},
            row! { cell!("Next Field"), cell!("<Tab>")},
            row! { cell!("Previous Field"), cell!("<Shift-Tab>")},
            row! { cell!("Update command"), cell!("<Enter/Ctrl-S>")},
            row! { cell!("Help"), cell!("<F1>")},
    }
}

fn insert_options() -> Table {
    table! {
            row! { cell!("Return"), cell!("<Esc/Ctrl-C>")},
            row! { cell!("Next Field"), cell!("<Tab>")},
            row! { cell!("Previous Field"), cell!("<Shift-Tab>")},
            row! { cell!("Create command"), cell!("<Enter/Ctrl-S>")},
            row! { cell!("Help"), cell!("<F1>")},
    }
}

#[derive(Clone)]
pub struct Table {
    pub content: Vec<Row>,
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.content
                .iter()
                .map(|row| row.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl Table {
    fn new(content: Vec<Row>) -> Table {
        Table { content }.build()
    }

    fn build(&self) -> Self {
        let row_bigger_cell_width = self.content.iter().fold(0, |mut acc, row| {
            let current_cell_width = row.width();
            if acc < current_cell_width {
                acc = current_cell_width
            }
            acc
        });

        let mut content = vec![];

        for row in &self.content {
            let new_row = row
                .cells()
                .map(|cell| {
                    let new_cell_content = cell.text.to_owned()
                        + &(" ").repeat((row_bigger_cell_width - cell.width()) as usize);

                    Cell::from(new_cell_content)
                })
                .collect::<Row>();

            content.push(new_row);
        }

        Table { content }
    }

    pub fn as_vec(&self) -> Vec<String> {
        self.content.iter().map(|r| r.to_string()).collect()
    }
}

impl From<Vec<Row>> for Table {
    fn from(content: Vec<Row>) -> Self {
        Table::new(content)
    }
}

impl Deref for Table {
    type Target = Vec<Row>;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

#[derive(Clone, Default)]
pub struct Row {
    pub cells: Vec<Cell>,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row = self
            .cells
            .iter()
            .map(|cell| cell.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{}", row)
    }
}

impl Row {
    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn width(&self) -> u16 {
        self.cells
            .iter()
            .map(|cell| cell.width())
            .max()
            .unwrap_or(0)
    }

    pub fn cells(&self) -> Iter<'_, Cell> {
        self.cells.iter()
    }
}

impl FromIterator<Cell> for Row {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        let mut row = Row::default();
        for i in iter {
            row.add_cell(i);
        }
        row
    }
}

#[derive(Clone)]
pub struct Cell {
    pub text: String,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Cell {
    fn new(text: String) -> Cell {
        Cell { text }
    }

    fn width(&self) -> u16 {
        self.text.width() as u16
    }
}

impl From<&'_ str> for Cell {
    fn from(text: &'_ str) -> Self {
        Cell::new(text.to_owned())
    }
}

impl From<String> for Cell {
    fn from(text: String) -> Self {
        Cell::new(text)
    }
}
