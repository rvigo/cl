use crate::gui::contexts::state::State;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::info;

use super::{
    contexts::popup_state::{Answer, MessageType},
    layouts::view_mode::ViewMode,
};

pub fn handle(key_event: KeyEvent, state: &mut State) {
    match state.view_mode {
        ViewMode::List => handle_list(key_event, state),
        ViewMode::New => handle_insert(key_event, state),
        ViewMode::Edit => handle_edit(key_event, state),
    }
}

pub fn handle_insert(key_event: KeyEvent, state: &mut State) {
    if state.popup_state.show_popup {
        handle_popup(key_event, state)
    } else {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            } => {
                info!("changing ViewMode to LIST");
                state.view_mode = ViewMode::List;
            }
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
            } => {
                state.ops_context.next();
            }

            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
            } => {
                state.ops_context.previous();
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => state.ops_context.get_current_in_focus().push(c),
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
            } => {
                state.ops_context.get_current_in_focus().pop();
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => match state.ops_context.build_command() {
                Ok(command) => match state.commands.add_command(&command) {
                    Ok(_) => {
                        state.reload_namespaces();
                        state.view_mode = ViewMode::List
                    }
                    Err(error) => {
                        state.popup_state.message_type = MessageType::Error;
                        state.popup_state.message = error.to_string();
                        state.popup_state.show_popup = true
                    }
                },
                Err(error) => {
                    state.popup_state.message_type = MessageType::Error;
                    state.popup_state.message = error.to_string();
                    state.popup_state.show_popup = true
                }
            },
            _ => {}
        }
    }
}

pub fn handle_edit(key_event: KeyEvent, state: &mut State) {
    if state.popup_state.show_popup {
        handle_popup(key_event, state)
    } else {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            } => {
                info!("changing ViewMode to LIST");
                state.view_mode = ViewMode::List;
            }
            KeyEvent {
                code: KeyCode::Right | KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
            } => {
                state.ops_context.next();
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
            } => {
                state.ops_context.previous();
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => state.ops_context.get_current_in_focus().push(c),
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
            } => {
                state.ops_context.get_current_in_focus().pop();
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                let current_command = state.ops_context.get_current_command().unwrap();
                let edited_command = state.ops_context.edit_command();

                match edited_command {
                    Ok(command) => match state
                        .commands
                        .add_edited_command(&command, &current_command)
                    {
                        Ok(_) => {
                            state.reload_namespaces();
                            state.view_mode = ViewMode::List
                        }
                        Err(error) => {
                            state.popup_state.message_type = MessageType::Error;
                            state.popup_state.message = error.to_string();
                            state.popup_state.show_popup = true
                        }
                    },
                    Err(error) => {
                        state.popup_state.message_type = MessageType::Error;
                        state.popup_state.message = error.to_string();
                        state.popup_state.show_popup = true
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn handle_list(key_event: KeyEvent, state: &mut State) {
    if state.popup_state.show_popup {
        handle_popup(key_event, state)
    } else {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
            } => {
                info!("shoul quit = true");
                state.should_quit = true;
            }
            KeyEvent {
                code: KeyCode::Left | KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
            } => {
                state.previous_namespace();
            }
            KeyEvent {
                code: KeyCode::Right | KeyCode::Char('l'),
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
            } => {
                state.next_namespace();
            }
            KeyEvent {
                code: KeyCode::Down | KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
            } => {
                state.next_command_item();
            }
            KeyEvent {
                code: KeyCode::Up | KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
            } => {
                state.previous_command_item();
            }
            KeyEvent {
                code: KeyCode::Insert | KeyCode::Char('i'),
                modifiers: KeyModifiers::NONE,
            } => {
                info!("changing ViewMode to NEW");
                state.view_mode = ViewMode::New;
            }
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::NONE,
            } => {
                info!("changing ViewMode to EDIT");
                state.view_mode = ViewMode::Edit;
                state
                    .get_mut_ref()
                    .ops_context
                    .set_selected_command_inputs();
            }

            KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::NONE,
            } => {
                info!("showing warning popup");
                state.popup_state.message =
                    String::from("Are you sure you want to delete the command?");
                state.popup_state.show_popup = true;
                state.popup_state.message_type = MessageType::Confirmation
            }

            _ => {}
        }
    }
}

fn handle_popup(key_event: KeyEvent, state: &mut State) {
    match state.popup_state.message_type {
        MessageType::Error => match key_event {
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                state.popup_state.message.clear();
                state.popup_state.show_popup = false;
                state.popup_state.message_type = MessageType::None
            }
            _ => {}
        },

        MessageType::Confirmation => match key_event {
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                match state
                    .commands
                    .remove(&state.ops_context.get_current_command().unwrap())
                {
                    Ok(_) => {
                        info!("the command was removed");
                        state.popup_state.message.clear();
                        state.popup_state.show_popup = false;
                        state.popup_state.message_type = MessageType::None;
                        state.popup_state.answer = Answer::None;
                    }
                    Err(error) => {
                        state.popup_state.message = error.to_string();
                        state.popup_state.message_type = MessageType::Error;
                        state.popup_state.answer = Answer::None;
                    }
                }
            }
            KeyEvent {
                code: KeyCode::Esc | KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
            } => {
                state.popup_state.message.clear();
                state.popup_state.show_popup = false;
                state.popup_state.message_type = MessageType::None
            }
            _ => {}
        },
        MessageType::None => {}
    }
}
