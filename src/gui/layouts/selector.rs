use std::io::Stdout;

use tui::{backend::CrosstermBackend, Frame};

use crate::gui::structs::{state::State, view_mode::ViewMode};

use super::{insert_layout, list_layout};

pub fn select_ui(frame: &mut Frame<CrosstermBackend<Stdout>>, state: &mut State) {
    match state.view_mode {
        ViewMode::List => list_layout::render(frame, state),
        ViewMode::New => insert_layout::render(frame, state),
    };
}
