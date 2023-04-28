use super::KeyEventHandler;
use crate::gui::{
    entities::events::app_events::{AppEvent, PopupEvent},
    screens::widgets::popup::MessageType,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub struct PopupHandler {
    message_type: Option<MessageType>,
}
impl PopupHandler {
    pub fn new(message_type: Option<MessageType>) -> PopupHandler {
        Self { message_type }
    }

    pub fn update_message_type(&mut self, message_type: Option<MessageType>) {
        self.message_type = message_type
    }
}

impl KeyEventHandler for PopupHandler {
    fn handle(&self, key_event: KeyEvent) -> Result<Option<AppEvent>> {
        if let Some(message_type) = &self.message_type {
            return match message_type {
                MessageType::Error => Ok(Some(AppEvent::Popup(PopupEvent::Disable))),
                MessageType::Warning => match key_event {
                    KeyEvent {
                        code: KeyCode::Right,
                        ..
                    } => Ok(Some(AppEvent::Popup(PopupEvent::NextChoice))),
                    KeyEvent {
                        code: KeyCode::Left,
                        ..
                    } => Ok(Some(AppEvent::Popup(PopupEvent::PreviousChoice))),
                    KeyEvent {
                        code: KeyCode::Enter,
                        ..
                    } => Ok(Some(AppEvent::Popup(PopupEvent::Answer))),
                    KeyEvent {
                        code: KeyCode::Esc | KeyCode::Char('q'),
                        ..
                    } => Ok(Some(AppEvent::Popup(PopupEvent::Disable))),
                    _ => Ok(None),
                },
            };
        }
        Ok(None)
    }
}
