use super::{
    contexts::popup::{Answer, MessageType},
    layouts::view_mode::ViewMode,
};
use crate::gui::contexts::state::State;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle(key_event: KeyEvent, state: &mut State) {
    match state.view_mode {
        ViewMode::List => handle_list(key_event, state),
        ViewMode::New => handle_insert(key_event, state),
        ViewMode::Edit => handle_edit(key_event, state),
    }
}

pub fn handle_insert(key_event: KeyEvent, state: &mut State) {
    if state.popup.show_popup {
        handle_popup(key_event, state)
    } else if state.show_help {
        handle_help(key_event, state)
    } else {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            } => {
                state.view_mode = ViewMode::List;
            }
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
            } => {
                state.context.next();
            }

            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
            } => {
                state.context.previous();
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => state.context.get_current_in_focus().push(c),
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
            } => {
                state.context.get_current_in_focus().pop();
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => match state.context.build_command() {
                Ok(command) => match state.commands.add_command(&command) {
                    Ok(_) => {
                        state.reload_namespaces();
                        state.view_mode = ViewMode::List
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
            } => state.show_help = true,
            _ => {}
        }
    }
}

pub fn handle_edit(key_event: KeyEvent, state: &mut State) {
    if state.popup.show_popup {
        handle_popup(key_event, state)
    } else if state.show_help {
        handle_help(key_event, state)
    } else {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            } => {
                state.view_mode = ViewMode::List;
            }
            KeyEvent {
                code: KeyCode::Right | KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
            } => {
                state.context.next();
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
            } => {
                state.context.previous();
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => state.context.get_current_in_focus().push(c),
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
            } => {
                state.context.get_current_in_focus().pop();
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                let current_command = state.context.get_current_command().unwrap();
                let edited_command = state.context.edit_command();

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
                }
            }
            KeyEvent {
                code: KeyCode::F(1),
                modifiers: KeyModifiers::NONE,
            } => state.show_help = true,
            _ => {}
        }
    }
}

pub fn handle_list(key_event: KeyEvent, state: &mut State) {
    if state.popup.show_popup {
        handle_popup(key_event, state)
    } else if state.show_help {
        handle_help(key_event, state)
    } else {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
            } => {
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
                state.view_mode = ViewMode::New;
            }
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::NONE,
            } => {
                state.view_mode = ViewMode::Edit;
                state.get_mut_ref().context.set_selected_command_inputs();
            }

            KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::NONE,
            } => {
                state.popup.message = String::from("Are you sure you want to delete the command?");
                state.popup.show_popup = true;
                state.popup.message_type = MessageType::Delete;
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                state.to_be_executed = state
                    .filtered_commands()
                    .get(state.commands_state.selected().unwrap())
                    .map(|i| i.to_owned());
                state.should_quit = true
            }
            KeyEvent {
                code: KeyCode::F(1),
                modifiers: KeyModifiers::NONE,
            } => state.show_help = true,
            _ => {}
        }
    }
}

fn handle_popup(key_event: KeyEvent, state: &mut State) {
    match state.popup.message_type {
        MessageType::Error => match key_event {
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                state.popup.clear();
            }
            _ => {}
        },

        MessageType::Delete => match key_event {
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
            } => state.popup.next(),
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
            } => state.popup.previous(),
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                state.popup.answer = state
                    .popup
                    .options
                    .get(state.popup.options_state.selected().unwrap())
                    .cloned()
                    .unwrap();
                match state.popup.answer {
                    Answer::Ok => {
                        match state
                            .commands
                            .remove(&state.context.get_current_command().unwrap())
                        {
                            Ok(_) => {
                                state.popup.clear();
                                state.reload_namespaces()
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
            } => {
                state.popup.clear();
            }
            _ => {}
        },
        MessageType::None => {}
    }
}

fn handle_help(key_event: KeyEvent, state: &mut State) {
    match key_event {
        _ => state.show_help = false,
    }
}
