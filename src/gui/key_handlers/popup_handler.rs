use super::Handler;
use crate::{
    gui::widgets::popup::{Answer, MessageType},
    resources::file_service,
};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Default)]
pub struct PopupHandler;

impl Handler for PopupHandler {
    fn handle(
        &self,
        key_event: crossterm::event::KeyEvent,
        state: &mut crate::gui::entities::state::State,
    ) {
        match state
            .popup_context
            .popup
            .as_ref()
            .unwrap()
            .message_type
            .as_ref()
            .unwrap()
        {
            MessageType::Error => {
                if let KeyEvent {
                    code: KeyCode::Enter,
                    ..
                } = key_event
                {
                    state.popup_context.clear();
                }
            }

            MessageType::Delete => {
                let choices = state.popup_context.popup.as_ref().unwrap().choices.clone();
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
                        match state
                            .popup_context
                            .popup
                            .as_ref()
                            .unwrap()
                            .choices
                            .get(state.popup_context.choices_state.selected().unwrap())
                            .unwrap()
                        {
                            Answer::Ok => {
                                match state
                                    .commands
                                    .remove(state.form_fields_context.selected_command().unwrap())
                                {
                                    Ok(commands) => {
                                        if let Ok(()) =
                                            file_service::write_to_command_file(commands)
                                        {
                                            state.popup_context.clear();
                                            state.reload_state();
                                        }
                                    }
                                    Err(error) => {
                                        state.popup_context.clear();
                                        log::error!("Something went wrong: {error}")
                                    }
                                }
                            }
                            Answer::Cancel => {
                                state.popup_context.clear();
                                if let Some(popup) = &mut state.popup_context.popup {
                                    popup.clear();
                                }
                            }
                        }
                    }
                    KeyEvent {
                        code: KeyCode::Esc | KeyCode::Char('q'),
                        ..
                    } => {
                        state.popup_context.clear();
                    }
                    _ => {}
                }
            }
        }
    }
}
