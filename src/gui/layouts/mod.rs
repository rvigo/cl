mod form_layout;
mod main_layout;

use parking_lot::Mutex;
use std::{io::Stdout, sync::Arc};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use super::entities::{
    application_context::ApplicationContext,
    ui_state::{UiState, ViewMode},
};

pub const DEFAULT_TEXT_COLOR: Color = Color::Rgb(229, 229, 229);
pub const DEFAULT_SELECTED_COLOR: Color = Color::Rgb(201, 165, 249);

#[derive(Debug, Clone, PartialEq, Default)]
pub enum TerminalSize {
    Small,
    #[default]
    Medium,
    Large,
}

pub fn get_terminal_size<B: Backend>(frame: &Frame<B>) -> TerminalSize {
    let size = frame.size();
    if size.height <= 20 {
        TerminalSize::Small
    } else if size.height <= 30 {
        TerminalSize::Medium
    } else {
        TerminalSize::Large
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
    let height = if height > 100 { 100 } else { height };
    let width = if width > 100 { 100 } else { width };

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

pub fn get_default_block<'a, T>(title: T) -> Block<'a>
where
    T: Into<String>,
{
    Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title(format!(" {} ", title.into()))
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Plain)
}

pub fn select_ui(
    frame: &mut Frame<CrosstermBackend<Stdout>>,
    ui_state: &mut Arc<Mutex<UiState>>,
    context: &mut Arc<Mutex<ApplicationContext>>,
) {
    let mut ui_state = ui_state.lock();
    let current_terminal_size = get_terminal_size(frame);
    if ui_state.size != current_terminal_size {
        ui_state.size = current_terminal_size;
        context.lock().reorder_fields(ui_state.size.to_owned());
    }

    match ui_state.view_mode {
        ViewMode::Main => main_layout::render(frame, context, &ui_state),
        ViewMode::Edit | ViewMode::Insert => form_layout::render(frame, context, &ui_state),
    }
}
