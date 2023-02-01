use super::Handler;
use crate::{
    gui::{
        entities::state::State,
        widgets::popup::{Answer, MessageType},
    },
    resources::file_service,
};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Default)]
pub struct PopupHandler;

impl Handler for PopupHandler {
    fn handle(&self, key_event: KeyEvent, state: &mut State) {
        if let Some(popup) = state.popup_context.popup.as_mut() {
            if let Some(message_type) = popup.message_type.as_ref() {
                match message_type {
                    MessageType::Error => handle_error_message_type(key_event, state),

                    MessageType::Delete => {
                        let choices = popup.choices.clone();
                        match key_event {
                            KeyEvent {
                                code: KeyCode::Right,
                                ..
                            } => state.popup_context.choices_state.next(choices),
                            KeyEvent {
                                code: KeyCode::Left,
                                ..
                            } => state.popup_context.choices_state.previous(choices),
                            KeyEvent {
                                code: KeyCode::Enter,
                                ..
                            } => {
                                if let Some(selected_choice_idx) =
                                    state.popup_context.choices_state.selected()
                                {
                                    if let Some(answer) = popup.choices.get(selected_choice_idx) {
                                        match answer {
                                            Answer::Ok => {
                                                if let Some(command) =
                                                    state.form_fields_context.selected_command()
                                                {
                                                    match state.commands.remove(command) {
                                                        Ok(commands) => {
                                                            if let Ok(()) =
                                                                file_service::write_to_command_file(
                                                                    commands,
                                                                )
                                                            {
                                                                state.popup_context.clear();
                                                                state.reload_state();
                                                            }
                                                        }
                                                        Err(error) => {
                                                            state.popup_context.clear();
                                                            log::error!(
                                                                "Something went wrong: {error}"
                                                            )
                                                        }
                                                    }
                                                }
                                            }
                                            Answer::Cancel => {
                                                popup.clear();
                                                state.popup_context.clear();
                                            }
                                        }
                                    }
                                }
                            }
                            KeyEvent {
                                code: KeyCode::Esc | KeyCode::Char('q'),
                                ..
                            } => handle_quit(state),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

fn handle_error_message_type(key_event: KeyEvent, state: &mut State) {
    if let KeyEvent {
        code: KeyCode::Enter,
        ..
    } = key_event
    {
        state.popup_context.clear();
    }
}

fn handle_quit(state: &mut State) {
    state.popup_context.clear();
}
