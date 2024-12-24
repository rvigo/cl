use crate::widget::popup::{Choice, Popup};
use crate::{event::PopupCallbackAction, widget::popup::Type, ViewMode};
use std::fmt;
use std::ops::Deref;
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

		Popup::new(content.to_string(), choices, Type::Help, PopupCallbackAction::None)
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

// TODO create a separator based on the max width of the largest cell and calculate the diff in each row
#[derive(Clone)]
pub struct Table<'a> {
	pub content: Vec<Row<'a>>,
}

impl fmt::Display for Table<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.content.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("\n"))
	}
}

impl Table<'_> {
	pub fn width(&self) -> u16 {
		self.content.iter().map(|row| row.width()).max().unwrap_or(0)
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
	pub cells: Vec<Cell<'a>>,
}

impl fmt::Display for Row<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let x = self.cells.iter().map(|s| s.to_string()).collect::<Vec<String>>().join("|");
		write!(f, "{x}")
	}
}

impl<'a> Row<'a> {
	pub fn add_cell(&mut self, cell: Cell<'a>) {
		self.cells.push(cell);
	}

	pub fn width(&self) -> u16 {
		self.cells.iter().map(|cell| cell.width()).max().unwrap_or(0)
	}
}

#[derive(Clone)]
pub struct Cell<'a> {
	pub text: &'a str,
}

impl fmt::Display for Cell<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:width$}", self.text, width = self.width() as usize)
	}
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

		let current = vec![&cell1, &cell2].iter().map(|cell| cell.width()).max().unwrap_or(0);
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

	#[test]
	fn should_center_the_table() {
		let cell1 = Cell::from("Quit");
		let cell2 = Cell::from("Move to previous namespace");

		let r = row! {
				cell1,
				cell2,
		};

		let table: Table<'_> = table! {
				r,
		};

		let table_string: String = main_options().to_string();

		println!("{}", table_string);
		// assert_eq!(table_width, 26);
		// assert_eq!(table_string_width, 26);
	}
}
