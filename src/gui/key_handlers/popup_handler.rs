use crate::gui::{
    entities::{
        application_context::ApplicationContext,
        events::app_events::{AppEvents, PopupEvent},
    },
    widgets::popup::MessageType,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use parking_lot::Mutex;
use std::sync::Arc;

pub fn handle(
    key_event: KeyEvent,
    context: &mut Arc<Mutex<ApplicationContext>>,
) -> Result<Option<AppEvents>> {
    let mut c = context.lock();
    if let Some(popup) = c.popup().as_mut() {
        if let Some(message_type) = popup.message_type().as_ref() {
            match message_type {
                MessageType::Error => return Ok(Some(AppEvents::Popup(PopupEvent::Disable))),
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
                    } => {
                        let answer = c.get_selected_choice();
                        return Ok(Some(AppEvents::Popup(PopupEvent::Answer(answer))));
                    }
                    KeyEvent {
                        code: KeyCode::Esc | KeyCode::Char('q'),
                        ..
                    } => {
                        return Ok(Some(AppEvents::Popup(PopupEvent::Disable)));
                    }
                    _ => {}
                },
            }
        }
    }

    Ok(None)
}

pub fn handle_help() -> Result<Option<AppEvents>> {
    Ok(Some(AppEvents::Popup(PopupEvent::Disable)))
}
