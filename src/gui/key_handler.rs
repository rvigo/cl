use super::{
    entities::{
        context::Context,
        popup::{Answer, MessageType},
    },
    layouts::view_mode::ViewMode,
};
use crate::{gui::entities::state::State, resources::file_service::CommandFileService};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct KeyHandler {
    file_service: CommandFileService,
}

impl KeyHandler {
    pub fn new(file_service: CommandFileService) -> KeyHandler {
        KeyHandler { file_service }
    }

    pub fn handle(&self, key_event: KeyEvent, state: &mut State) {
        match state.view_mode {
            ViewMode::List => self.handle_list(key_event, state),
            ViewMode::Insert => self.handle_insert(key_event, state),
            ViewMode::Edit => self.handle_edit(key_event, state),
        }
    }

    pub fn handle_insert(&self, key_event: KeyEvent, state: &mut State) {
        if state.popup.show_popup {
            self.handle_popup(key_event, state)
        } else if state.show_help {
            self.handle_help(state)
        } else {
            match key_event {
                KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                } => {
                    state.context.clear_inputs();
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
                } => state.context.get_current_in_focus_mut().unwrap().on_char(c),
                KeyEvent {
                    code: KeyCode::Backspace,
                    modifiers: KeyModifiers::NONE,
                } => {
                    state
                        .context
                        .get_current_in_focus_mut()
                        .unwrap()
                        .on_backspace();
                }
                KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                } => {
                    state
                        .context
                        .get_current_in_focus_mut()
                        .unwrap()
                        .move_cursor_backward();
                }
                KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                } => {
                    state
                        .context
                        .get_current_in_focus_mut()
                        .unwrap()
                        .move_cursor_foward();
                }
                KeyEvent {
                    code: KeyCode::Delete,
                    modifiers: KeyModifiers::NONE,
                } => state
                    .context
                    .get_current_in_focus_mut()
                    .unwrap()
                    .on_delete_key(),
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                } => match state.context.build_command() {
                    Ok(command) => match state.commands.add_command(&command) {
                        Ok(items) => {
                            if let Ok(()) = self.file_service.write_to_command_file(items) {
                                state.reload_state();
                                state.view_mode = ViewMode::List
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

    pub fn handle_edit(&self, key_event: KeyEvent, state: &mut State) {
        if state.popup.show_popup {
            self.handle_popup(key_event, state)
        } else if state.show_help {
            self.handle_help(state)
        } else {
            match key_event {
                KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                } => {
                    state.context.clear_inputs();
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
                } => state.context.get_current_in_focus_mut().unwrap().on_char(c),
                KeyEvent {
                    code: KeyCode::Backspace,
                    modifiers: KeyModifiers::NONE,
                } => {
                    state
                        .context
                        .get_current_in_focus_mut()
                        .unwrap()
                        .on_backspace();
                }
                KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                } => {
                    state
                        .context
                        .get_current_in_focus_mut()
                        .unwrap()
                        .move_cursor_backward();
                }
                KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                } => {
                    state
                        .context
                        .get_current_in_focus_mut()
                        .unwrap()
                        .move_cursor_foward();
                }
                KeyEvent {
                    code: KeyCode::Delete,
                    modifiers: KeyModifiers::NONE,
                } => state
                    .context
                    .get_current_in_focus_mut()
                    .unwrap()
                    .on_delete_key(),
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                } => {
                    let context: &mut Context = &mut state.context;
                    let current_command = context.get_current_command().unwrap().clone();
                    let edited_command = context.edit_command();

                    match edited_command {
                        Ok(command) => match state
                            .commands
                            .add_edited_command(&command, &current_command)
                        {
                            Ok(items) => {
                                if let Ok(()) = self.file_service.write_to_command_file(items) {
                                    state.reload_state();
                                    state.view_mode = ViewMode::List
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

    pub fn handle_list(&self, key_event: KeyEvent, state: &mut State) {
        if state.popup.show_popup {
            self.handle_popup(key_event, state)
        } else if state.show_help {
            self.handle_help(state)
        }
        if state.find_flag {
            self.handle_find(key_event, state)
        } else {
            match key_event {
                KeyEvent {
                    code: KeyCode::Char('f'),
                    modifiers: KeyModifiers::NONE,
                } => {
                    //unlock find frame
                    state.set_find_active()
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
                    state.view_mode = ViewMode::Insert;
                }
                KeyEvent {
                    code: KeyCode::Char('e'),
                    modifiers: KeyModifiers::NONE,
                } => {
                    state.view_mode = ViewMode::Edit;
                    state.context.set_selected_command_input();
                }

                KeyEvent {
                    code: KeyCode::Char('d') | KeyCode::Delete,
                    modifiers: KeyModifiers::NONE,
                } => {
                    state.popup.message =
                        String::from("Are you sure you want to delete the command?");
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
                        .cloned();
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

    fn handle_popup(&self, key_event: KeyEvent, state: &mut State) {
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
                                .remove(state.context.get_current_command().unwrap())
                            {
                                Ok(items) => {
                                    if let Ok(()) = self.file_service.write_to_command_file(items) {
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

    fn handle_help(&self, state: &mut State) {
        state.show_help = false;
    }

    fn handle_find(&self, key_event: KeyEvent, state: &mut State) {
        match key_event {
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
            } => {
                state.query_string.push(c);
            }
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
            } => {
                state.query_string.pop();
            }

            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            } => state.set_find_deactive(),
            _ => {}
        }
    }
}
