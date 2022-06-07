use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::info;

use crate::gui::structs::state::State;

pub fn handle(key_event: KeyEvent, state: &mut State) -> bool {
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
            modifiers: KeyModifiers::NONE,
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
            state.next();
            false
        }
        KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        } => {
            state.previous();
            false
        }

        KeyEvent {
            code: KeyCode::Char('r'),
            modifiers: KeyModifiers::CONTROL,
        } => {
            info!("executing");
            false
        }
        _ => false,
    }
}
