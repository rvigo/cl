use crate::{
    gui::entities::{
        popup::{Answer, MessageType},
        state::State,
    },
    resources::file_service,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_popup(key_event: KeyEvent, state: &mut State) {
    match state.popup.message_type {
        MessageType::Error => {
            if let KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } = key_event
            {
                state.popup.clear();
            }
        }

        MessageType::Delete => match key_event {
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            } => state.popup.next(),
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            } => state.popup.previous(),
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                match state
                    .popup
                    .options
                    .get(state.popup.options_state.selected().unwrap())
                    .unwrap()
                {
                    Answer::Ok => {
                        match state
                            .commands
                            .remove(state.field_context.get_current_command().unwrap())
                        {
                            Ok(commands) => {
                                if let Ok(()) = file_service::write_to_command_file(commands) {
                                    state.popup.clear();
                                    state.reload_state();
                                }
                            }
                            Err(error) => {
                                state.popup.clear();
                                state.popup.message = error.to_string();
                            }
                        }
                    }
                    Answer::Cancel => {
                        state.popup.clear();
                    }
                    _ => {}
                }
            }
            KeyEvent {
                code: KeyCode::Esc | KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state.popup.clear();
            }
            _ => {}
        },
        MessageType::None => {}
    }
}

pub fn handle_help(state: &mut State) {
    state.show_help = false;
}

pub fn handle_query_box(key_event: KeyEvent, state: &mut State) {
    match key_event {
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            ..
        } => {
            state.query_box.on_char(c);
            state.reload_state()
        }
        KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
            ..
        } => {
            state.query_box.on_backspace();
            state.reload_state()
        }
        KeyEvent {
            code: KeyCode::Delete,
            modifiers: KeyModifiers::NONE,
            ..
        } => {
            state.query_box.on_delete_key();
            state.reload_state()
        }
        KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
            ..
        } => {
            state.query_box.move_cursor_backward();
        }
        KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
            ..
        } => {
            state.query_box.move_cursor_foward();
        }
        KeyEvent {
            code: KeyCode::Esc | KeyCode::Enter | KeyCode::Down | KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            ..
        } => state.query_box.toggle_focus(),
        _ => {}
    }
}
