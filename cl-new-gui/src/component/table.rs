use std::fmt;
use std::ops::Deref;
use std::slice::Iter;
use unicode_width::UnicodeWidthStr;

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
        let row_bigger_cell_width = self.content.iter().fold(0u16, |acc, row| {
            acc.max(row.width())
        });

        let content = self
            .content
            .iter()
            .map(|row| {
                row.cells()
                    .map(|cell| {
                        let padded = cell.text.to_owned()
                            + &" ".repeat((row_bigger_cell_width - cell.width()) as usize);
                        Cell::from(padded)
                    })
                    .collect::<Row>()
            })
            .collect();

        Table { content }
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

        write!(f, "{row}")
    }
}

impl Row {
    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.push(cell);
    }

    pub fn width(&self) -> u16 {
        self.cells.iter().map(|cell| cell.width()).max().unwrap_or(0)
    }

    pub fn cells(&self) -> Iter<'_, Cell> {
        self.cells.iter()
    }
}

impl FromIterator<Cell> for Row {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        let mut row = Row::default();
        for cell in iter {
            row.add_cell(cell);
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

    pub fn width(&self) -> u16 {
        self.text.custom_width()
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

pub trait CustomWidth {
    fn custom_width(&self) -> u16;
}

impl CustomWidth for str {
    fn custom_width(&self) -> u16 {
        self.lines().fold(0u16, |acc, line| acc.max(line.width() as u16))
    }
}
