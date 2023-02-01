mod form_layout;
mod main_layout;

use crate::gui::entities::state::State;
use std::{fmt, io::Stdout};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub const DEFAULT_TEXT_COLOR: Color = Color::Rgb(229, 229, 229);
pub const DEFAULT_SELECTED_COLOR: Color = Color::Rgb(201, 165, 249);

pub fn select_ui(frame: &mut Frame<CrosstermBackend<Stdout>>, state: &mut State) {
    match state.view_mode {
        ViewMode::Main => main_layout::render(frame, state),
        ViewMode::Insert | ViewMode::Edit => form_layout::render(frame, state),
    };
}

#[derive(Clone, Default, PartialEq, Eq)]
pub enum ViewMode {
    #[default]
    Main,
    Insert,
    Edit,
}

impl fmt::Display for ViewMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ViewMode::Main => write!(f, "Main"),
            ViewMode::Insert => write!(f, "Insert"),
            ViewMode::Edit => write!(f, "Edit"),
        }
    }
}

pub fn get_style(in_focus: bool) -> Style {
    if in_focus {
        Style::default().fg(Color::Black).bg(DEFAULT_SELECTED_COLOR)
    } else {
        Style::default().fg(DEFAULT_TEXT_COLOR)
    }
}

pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let new_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - height) / 2),
                Constraint::Percentage(height),
                Constraint::Percentage((100 - height) / 2),
            ]
            .as_ref(),
        )
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - width) / 2),
                Constraint::Percentage(width),
                Constraint::Percentage((100 - width) / 2),
            ]
            .as_ref(),
        )
        .split(new_area[1])[1]
}

pub fn get_default_block<'a>(title: String) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title(format!(" {title} "))
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Plain)
}
