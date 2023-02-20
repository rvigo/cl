use super::KeyHandler;
use crate::gui::{entities::application_context::ApplicationContext, widgets::popup::MessageType};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Default)]
pub struct PopupHandler;

impl KeyHandler for PopupHandler {
    fn handle(&self, key_event: KeyEvent, context: &mut ApplicationContext) {
        if let Some(popup) = context.popup().as_mut() {
            if let Some(message_type) = popup.message_type().as_ref() {
                match message_type {
                    MessageType::Error => handle_error_message_type(key_event, context),

                    MessageType::Warning => match key_event {
                        KeyEvent {
                            code: KeyCode::Right,
                            ..
                        } => context.next_popup_choice(),
                        KeyEvent {
                            code: KeyCode::Left,
                            ..
                        } => context.previous_popup_choice(),
                        KeyEvent {
                            code: KeyCode::Enter,
                            ..
                        } => context.handle_warning_popup(),
                        KeyEvent {
                            code: KeyCode::Esc | KeyCode::Char('q'),
                            ..
                        } => handle_quit(context),
                        _ => {}
                    },
                }
            }
        }
    }
}

fn handle_error_message_type(key_event: KeyEvent, context: &mut ApplicationContext) {
    if let KeyEvent {
        code: KeyCode::Enter,
        ..
    } = key_event
    {
        context.clear_popup_context();
    }
}

fn handle_quit(context: &mut ApplicationContext) {
    context.clear_popup_context();
}
