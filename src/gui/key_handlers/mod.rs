mod edit_handler;
mod handler_utils;
mod insert_handler;
mod main_handler;

use crate::gui::{entities::state::State, layouts::view_mode::ViewMode};
use crossterm::event::KeyEvent;

pub fn handle(key_event: KeyEvent, state: &mut State) {
    match state.view_mode {
        ViewMode::Main => main_handler::handle(key_event, state),
        ViewMode::Insert => insert_handler::handle(key_event, state),
        ViewMode::Edit => edit_handler::handle(key_event, state),
    }
}
