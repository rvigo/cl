use super::handler_utils::{handle_help, handle_popup, handle_query_box};
use crate::gui::{
    entities::{popup::MessageType, state::State},
    layouts::view_mode::ViewMode,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle(key_event: KeyEvent, state: &mut State) {
    if state.popup.show_popup {
        handle_popup(key_event, state)
    } else if state.show_help {
        handle_help(state)
    } else if state.query_box.in_focus() {
        handle_query_box(key_event, state)
    } else {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::NONE,
                ..
            } => state.query_box.toggle_focus(),
            KeyEvent {
                code: KeyCode::Char('q') | KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state.should_quit = true;
            }
            KeyEvent {
                code: KeyCode::Left | KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                state.previous_namespace();
            }
            KeyEvent {
                code: KeyCode::Right | KeyCode::Char('l'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state.next_namespace();
            }
            KeyEvent {
                code: KeyCode::Down | KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state.next_command();
            }
            KeyEvent {
                code: KeyCode::Up | KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state.previous_command();
            }
            KeyEvent {
                code: KeyCode::Insert | KeyCode::Char('i'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state.view_mode = ViewMode::Insert;
            }
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if !state
                    .field_context
                    .get_current_command()
                    .unwrap()
                    .is_empty()
                {
                    state.view_mode = ViewMode::Edit;
                    state.field_context.set_selected_command_input();
                }
            }

            KeyEvent {
                code: KeyCode::Char('d') | KeyCode::Delete,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if !state
                    .field_context
                    .get_current_command()
                    .unwrap()
                    .is_empty()
                {
                    state.popup.message =
                        String::from("Are you sure you want to delete the command?");
                    state.popup.show_popup = true;
                    state.popup.message_type = MessageType::Delete;
                }
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if !state
                    .field_context
                    .get_current_command()
                    .unwrap()
                    .is_empty()
                {
                    state.to_be_executed = state
                        .filter_commands()
                        .get(state.commands_state.selected().unwrap())
                        .cloned();
                    state.should_quit = true
                }
            }
            KeyEvent {
                code: KeyCode::F(1) | KeyCode::Char('?'),
                modifiers: KeyModifiers::NONE,
                ..
            } => state.show_help = true,
            _ => {}
        }
    }
}
