use crate::gui::{
    entities::{application_context::ApplicationContext, events::app_events::AppEvents},
    widgets::popup::MessageType,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use parking_lot::{lock_api::MutexGuard, Mutex, RawMutex};
use std::sync::Arc;

pub fn handle(
    key_event: KeyEvent,
    context: &mut Arc<Mutex<ApplicationContext>>,
) -> Result<Option<AppEvents>> {
    let mut c = context.lock();
    if let Some(popup) = c.popup().as_mut() {
        if let Some(message_type) = popup.message_type().as_ref() {
            match message_type {
                MessageType::Error => handle_error_message_type(key_event, &mut c),

                MessageType::Warning => match key_event {
                    KeyEvent {
                        code: KeyCode::Right,
                        ..
                    } => c.next_popup_choice(),
                    KeyEvent {
                        code: KeyCode::Left,
                        ..
                    } => c.previous_popup_choice(),
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => c.handle_warning_popup(),
                    KeyEvent {
                        code: KeyCode::Esc | KeyCode::Char('q'),
                        ..
                    } => handle_quit(&mut c),
                    _ => {}
                },
            }
        }
    }

    Ok(None)
}

fn handle_error_message_type(
    key_event: KeyEvent,
    context: &mut MutexGuard<RawMutex, ApplicationContext>,
) {
    if let KeyEvent {
        code: KeyCode::Enter,
        ..
    } = key_event
    {
        context.clear_popup_context();
    }
}

fn handle_quit(context: &mut MutexGuard<RawMutex, ApplicationContext>) {
    context.clear_popup_context();
}
