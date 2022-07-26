use super::{edit_layout, insert_layout, list_layout, view_mode::ViewMode};
use crate::gui::contexts::state::State;
use std::io::Stdout;
use tui::{backend::CrosstermBackend, Frame};

pub fn select_ui(frame: &mut Frame<CrosstermBackend<Stdout>>, state: &mut State) {
    match state.view_mode {
        ViewMode::List => list_layout::render(frame, state),
        ViewMode::Insert => insert_layout::render(frame, state),
        ViewMode::Edit => edit_layout::render(frame, state),
    };
}
