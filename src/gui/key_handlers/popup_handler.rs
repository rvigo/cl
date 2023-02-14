use super::KeyHandler;
use crate::{
    gui::{
        entities::application_context::ApplicationContext,
        widgets::popup::{Answer, MessageType},
    },
    resources::file_service,
};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Default)]
pub struct PopupHandler;

impl KeyHandler for PopupHandler {
    fn handle(&self, key_event: KeyEvent, application_context: &mut ApplicationContext) {
        if let Some(popup) = application_context.ui_context.popup_context.popup.as_mut() {
            if let Some(message_type) = popup.message_type().as_ref() {
                match message_type {
                    MessageType::Error => handle_error_message_type(key_event, application_context),

                    MessageType::Warning => {
                        let choices = popup.choices();
                        match key_event {
                            KeyEvent {
                                code: KeyCode::Right,
                                ..
                            } => application_context
                                .ui_context
                                .popup_context
                                .choices_state
                                .next(choices),
                            KeyEvent {
                                code: KeyCode::Left,
                                ..
                            } => application_context
                                .ui_context
                                .popup_context
                                .choices_state
                                .previous(choices),
                            KeyEvent {
                                code: KeyCode::Enter,
                                ..
                            } => {
                                if let Some(selected_choice_idx) = application_context
                                    .ui_context
                                    .popup_context
                                    .choices_state
                                    .selected()
                                {
                                    if let Some(answer) = popup.choices().get(selected_choice_idx) {
                                        match answer {
                                            Answer::Ok => {
                                                if let Some(command) = application_context
                                                    .ui_context
                                                    .form_fields_context
                                                    .selected_command()
                                                {
                                                    match application_context
                                                        .commands
                                                        .remove(command)
                                                    {
                                                        Ok(commands) => {
                                                            if let Ok(()) =
                                                                file_service::write_to_command_file(
                                                                    commands,
                                                                )
                                                            {
                                                                application_context
                                                                    .ui_context
                                                                    .popup_context
                                                                    .clear();
                                                                application_context.reload_state();
                                                            }
                                                        }
                                                        Err(error) => {
                                                            application_context
                                                                .ui_context
                                                                .popup_context
                                                                .clear();
                                                            log::error!(
                                                                "Something went wrong: {error}"
                                                            )
                                                        }
                                                    }
                                                }
                                            }
                                            Answer::Cancel => {
                                                application_context
                                                    .ui_context
                                                    .popup_context
                                                    .clear();
                                            }
                                        }
                                    }
                                }
                            }
                            KeyEvent {
                                code: KeyCode::Esc | KeyCode::Char('q'),
                                ..
                            } => handle_quit(application_context),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

fn handle_error_message_type(key_event: KeyEvent, application_context: &mut ApplicationContext) {
    if let KeyEvent {
        code: KeyCode::Enter,
        ..
    } = key_event
    {
        application_context.ui_context.popup_context.clear();
    }
}

fn handle_quit(application_context: &mut ApplicationContext) {
    application_context.ui_context.popup_context.clear();
}
