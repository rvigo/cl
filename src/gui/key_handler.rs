use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::info;

use crate::gui::structs::{state::State, view_mode::ViewMode};

pub fn handle(key_event: KeyEvent, state: &mut State) -> bool {
    match state.view_mode {
        ViewMode::List => handle_list(key_event, state),
        ViewMode::New => handle_insert(key_event, state),
    }
}

pub fn handle_insert(key_event: KeyEvent, state: &mut State) -> bool {
    match key_event {
        KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
        } => {
            info!("changing ViewMode to LIST");
            state.view_mode = ViewMode::List;
            false
        }
        _ => false,
    }
}

pub fn handle_list(key_event: KeyEvent, state: &mut State) -> bool {
    match key_event {
        KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
        } => true,
        KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
        }
        | KeyEvent {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::SHIFT,
        } => {
            state.previous_namespace();
            false
        }
        KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
        }
        | KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::NONE,
        } => {
            state.next_namespace();
            false
        }
        KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
        } => {
            state.next_command_item();
            false
        }
        KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        } => {
            state.previous_command_item();
            false
        }
        KeyEvent {
            code: KeyCode::Insert,
            modifiers: KeyModifiers::NONE,
        } => {
            info!("changing ViewMode to NEW");
            state.view_mode = ViewMode::New;
            false
        }
        _ => false,
    }
}
