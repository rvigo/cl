use crate::gui::{
    entities::{
        events::app_events::{AppEvents, PopupEvent},
        ui_context::UIContext,
    },
    widgets::popup::MessageType,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use parking_lot::Mutex;
use std::sync::Arc;

pub fn handle(
    key_event: KeyEvent,
    ui_context: &mut Arc<Mutex<UIContext<'static>>>,
) -> Result<Option<AppEvents>> {
    let mut ui = ui_context.lock();
    if let Some(popup) = ui.popup().as_mut() {
        if let Some(message_type) = popup.message_type().as_ref() {
            match message_type {
                MessageType::Error => return Ok(Some(AppEvents::Popup(PopupEvent::Disable))),
                MessageType::Warning => match key_event {
                    KeyEvent {
                        code: KeyCode::Right,
                        ..
                    } => ui.next_choice(),
                    KeyEvent {
                        code: KeyCode::Left,
                        ..
                    } => ui.previous_choice(),
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => {
                        let answer = ui.get_selected_choice();
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
