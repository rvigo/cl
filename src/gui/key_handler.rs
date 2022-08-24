use super::{
    entities::{
        fields_context::FieldsContext,
        popup::{Answer, MessageType},
    },
    layouts::view_mode::ViewMode,
};
use crate::{gui::entities::state::State, resources::file_service};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle(key_event: KeyEvent, state: &mut State) {
    match state.view_mode {
        ViewMode::Main => handle_main(key_event, state),
        ViewMode::Insert => handle_insert(key_event, state),
        ViewMode::Edit => handle_edit(key_event, state),
    }
}

pub fn handle_insert(key_event: KeyEvent, state: &mut State) {
    if state.popup.show_popup {
        handle_popup(key_event, state)
    } else if state.show_help {
        handle_help(state)
    } else {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            } => {
                state.field_context.clear_inputs();
                state.view_mode = ViewMode::Main;
            }
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
            } => {
                state.field_context.next();
            }
            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
            } => {
                state.field_context.previous();
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => state
                .field_context
                .get_current_in_focus_mut()
                .unwrap()
                .on_char(c),
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
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
            } => state
                .field_context
                .get_current_in_focus_mut()
                .unwrap()
                .on_delete_key(),
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
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
            } => state.show_help = true,
            _ => {}
        }
    }
}

pub fn handle_edit(key_event: KeyEvent, state: &mut State) {
    if state.popup.show_popup {
        handle_popup(key_event, state)
    } else if state.show_help {
        handle_help(state)
    } else {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            } => {
                state.field_context.clear_inputs();
                state.view_mode = ViewMode::Main;
            }
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
            } => {
                state.field_context.next();
            }
            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
            } => {
                state.field_context.previous();
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => state
                .field_context
                .get_current_in_focus_mut()
                .unwrap()
                .on_char(c),
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
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
            } => state
                .field_context
                .get_current_in_focus_mut()
                .unwrap()
                .on_delete_key(),
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } => {
                let context: &mut FieldsContext = &mut state.field_context;
                let current_command = context.get_current_command().unwrap().clone();
                let edited_command = context.edit_command();

                match edited_command {
                    Ok(command) => match state
                        .commands
                        .add_edited_command(&command, &current_command)
                    {
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

pub fn handle_main(key_event: KeyEvent, state: &mut State) {
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
            } => {
                //unlock find frame
                state.query_box.toggle_focus()
            }
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
                code: KeyCode::Down | KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
            } => {
                state.next_command();
            }
            KeyEvent {
                code: KeyCode::Up | KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
            } => {
                state.previous_command();
            }
            KeyEvent {
                code: KeyCode::Insert | KeyCode::Char('i'),
                modifiers: KeyModifiers::NONE,
            } => {
                state.view_mode = ViewMode::Insert;
            }
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::NONE,
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
            } => state.show_help = true,
            _ => {}
        }
    }
}

fn handle_popup(key_event: KeyEvent, state: &mut State) {
    match state.popup.message_type {
        MessageType::Error => {
            if let KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
            } = key_event
            {
                state.popup.clear();
            }
        }

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
            } => {
                state.popup.clear();
            }
            _ => {}
        },
        MessageType::None => {}
    }
}

fn handle_help(state: &mut State) {
    state.show_help = false;
}

fn handle_query_box(key_event: KeyEvent, state: &mut State) {
    match key_event {
        KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
        } => {
            state.query_box.on_char(c);
            state.reload_state()
        }
        KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
        } => {
            state.query_box.on_backspace();
            state.reload_state()
        }
        KeyEvent {
            code: KeyCode::Delete,
            modifiers: KeyModifiers::NONE,
        } => {
            state.query_box.on_delete_key();
            state.reload_state()
        }
        KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
        } => {
            state.query_box.move_cursor_backward();
        }
        KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
        } => {
            state.query_box.move_cursor_foward();
        }
        KeyEvent {
            code: KeyCode::Esc | KeyCode::Enter | KeyCode::Down | KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        } => state.query_box.toggle_focus(),
        _ => {}
    }
}
