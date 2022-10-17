use super::{form_layout, main_layout, view_mode::ViewMode};
use crate::gui::entities::state::State;
use std::io::Stdout;
use tui::{backend::CrosstermBackend, Frame};

pub fn select_ui(frame: &mut Frame<CrosstermBackend<Stdout>>, state: &mut State) {
    match state.view_mode {
        ViewMode::Main => main_layout::render(frame, state),
        ViewMode::Insert | ViewMode::Edit => form_layout::render(frame, state),
    };
}
