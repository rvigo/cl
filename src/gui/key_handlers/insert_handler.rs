use super::handler_utils::{handle_help, handle_popup};
use crate::{
    gui::{
        entities::{popup::MessageType, state::State},
        layouts::view_mode::ViewMode,
    },
    resources::file_service,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle(key_event: KeyEvent, state: &mut State) {
    if state.popup.show_popup {
        handle_popup(key_event, state)
    } else if state.show_help {
        handle_help(state)
    } else {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state.field_context.clear_inputs();
                state.view_mode = ViewMode::Main;
            }
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state.field_context.next();
            }
            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                state.field_context.previous();
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                ..
            } => state
                .field_context
                .get_current_in_focus_mut()
                .unwrap()
                .on_char(c),
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state
                    .field_context
                    .get_current_in_focus_mut()
                    .unwrap()
                    .on_backspace();
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state
                    .field_context
                    .get_current_in_focus_mut()
                    .unwrap()
                    .move_cursor_backward();
            }
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state
                    .field_context
                    .get_current_in_focus_mut()
                    .unwrap()
                    .move_cursor_foward();
            }
            KeyEvent {
                code: KeyCode::Delete,
                modifiers: KeyModifiers::NONE,
                ..
            } => state
                .field_context
                .get_current_in_focus_mut()
                .unwrap()
                .on_delete_key(),
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => match state.field_context.build_command() {
                Ok(command) => match state.commands.add_command(&command) {
                    Ok(commands) => {
                        if let Ok(()) = file_service::write_to_command_file(commands) {
                            state.reload_state();
                            state.view_mode = ViewMode::Main
                        }
                    }
                    Err(error) => {
                        state.popup.message_type = MessageType::Error;
                        state.popup.message = error.to_string();
                        state.popup.show_popup = true
                    }
                },
                Err(error) => {
                    state.popup.message_type = MessageType::Error;
                    state.popup.message = error.to_string();
                    state.popup.show_popup = true
                }
            },
            KeyEvent {
                code: KeyCode::F(1),
                modifiers: KeyModifiers::NONE,
                ..
            } => state.show_help = true,
            _ => {}
        }
    }
}
