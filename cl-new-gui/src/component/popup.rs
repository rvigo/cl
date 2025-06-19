use crate::component::button::Button;
use crate::component::Renderable;
use crate::screen::theme::Theme;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::DeleteCommand;
use crate::{async_fn_body, oneshot, Pipe};
use anyhow::bail;
use log::debug;
use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use std::slice::Iter;
use tokio::sync::mpsc::Sender;
use tui::layout::Alignment::Center;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::widgets::{Block, Clear, Paragraph, Wrap};
use tui::Frame;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Default)]
pub struct PopupState {
    selected: usize,
}

impl PopupState {
    pub fn select(&mut self, index: usize) {
        self.selected = index;
    }
}

#[derive(Debug, Clone, Default)]
pub struct Popup {
    pub title: String,
    pub content: String,
    pub buttons: Vec<Button>,
    pub state: PopupState,
}

impl Popup {
    pub fn next(&mut self) {
        let current = self.state.selected;
        let next = (current + 1) % self.buttons.len();
        self.state.select(next);
    }

    pub fn previous(&mut self) {
        let current = self.state.selected;
        let previous = (current + self.buttons.len() - 1) % self.buttons.len();
        self.state.select(previous);
    }

    pub async fn click(&mut self, state_tx: Sender<StateEvent>) -> anyhow::Result<()> {
        if self.buttons.is_empty() {
            debug!("No buttons to click");
            return Ok(());
        }
        let selected = &self.buttons[self.state.selected];
        (selected.on_click)(state_tx).await
    }
}

impl Popup {
    pub fn dialog(message: String) -> Self {
        let mut popup = Popup::default();
        popup.title = "Warning".to_string();
        popup.content = message;
        popup.buttons = vec![
            Button::new("Yes", true, |state| {
                async_fn_body! {
                    let result = oneshot!(state, DeleteCommand);
                    match result{
                        // TODO handle error
                      Some((ok, reason)) => {
                            if !ok {
                                debug!("Something went wrong!");
                                bail!(reason.unwrap())
                            }
                            else {
                                debug!("Command deleted");
                                Ok(())
                            }
                        }
                      None => {
                            debug!("Something went wrong");
                        Ok(())
                        }
                    }
                }
            }),
            Button::new("No", false, |_| {
                async_fn_body! {
                    Ok(())
                }
            }),
        ];

        popup
    }

    pub fn help_main() -> Self {
        let mut popup = Popup::default();
        popup.title = "Help".to_string();
        popup.content = main_options().to_string(); // TODO rewrite this
        popup.buttons = vec![];

        popup
    }
}

impl Renderable for Popup {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let paragraph = Paragraph::new(self.content.to_owned())
            .alignment(Center)
            .style(
                Style::default()
                    .fg(theme.to_owned().text_color.into())
                    .bg(theme.to_owned().background_color.into()),
            )
            .wrap(Wrap { trim: true })
            .block(Block::bordered());

        let popup_area = compute_popup_area(&self.content, area);
        let [_, buttons_area] = *split_content_and_buttons(popup_area) else {
            panic!()
        };
        frame.render_widget(Clear, popup_area);
        frame.render_widget(paragraph, popup_area);

        // crate are for the buttons
        let button_area = button_area(self.buttons.len(), buttons_area);
        // render buttons inside that area
        button_area.iter().enumerate().for_each(|(i, area)| {
            let current_button = &mut self.buttons[i];
            if i == self.state.selected {
                current_button.is_active = true
            } else {
                current_button.is_active = false
            }
            current_button.render(frame, *area, theme);
        });
    }
}

fn button_area(number_of_buttons: usize, area: Rect) -> Rc<[Rect]> {
    if number_of_buttons == 0 {
        return Rc::from([]);
    }

    let constraints =
        vec![Constraint::Percentage(100 / number_of_buttons as u16); number_of_buttons];

    create_button_layout(area, &constraints)
}

fn create_button_layout(area: Rect, constraints: &[Constraint]) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area)
}

fn split_content_and_buttons(rect: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(rect)
}

fn compute_popup_area(content: &str, area: Rect) -> Rect {
    use tui::layout::{Constraint, Direction, Layout};

    let width = content.custom_width();
    let height = 5;

    const SCALE_FACTOR: u16 = 100;
    const MAX_SCALE_RATIO: f32 = 2.0;

    let scaled_height = (SCALE_FACTOR * (height * 2)) / area.height;
    let max_height = (area.height as f32 * MAX_SCALE_RATIO) as u16;
    let final_height = std::cmp::min(scaled_height, max_height);

    let height = if final_height > 100 {
        100
    } else {
        final_height
    };

    let width = if width > 50 { 50 } else { width };

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - height) / 2),
            Constraint::Percentage(height),
            Constraint::Percentage((100 - height) / 2),
        ])
        .split(area)[1]
        .pipe(|new_area| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage((100 - width) / 2),
                    Constraint::Percentage(width),
                    Constraint::Percentage((100 - width) / 2),
                ])
                .split(new_area)[1]
        })
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
macro_rules! table {
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
                        + &" ".repeat((row_bigger_cell_width - cell.width()) as usize);

                    Cell::from(new_cell_content)
                })
                .collect::<Row>();

            content.push(new_row);
        }

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
        self.text.custom_width() as u16
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

trait CustomWidth {
    fn custom_width(&self) -> u16;
}

impl CustomWidth for str {
    fn custom_width(&self) -> u16 {
        let lines = self.lines();
        lines.into_iter().fold(0, |mut acc, line| {
            let cur_row_width = line.width() as u16;
            if acc < cur_row_width {
                acc = cur_row_width
            };

            acc
        })
    }
}
