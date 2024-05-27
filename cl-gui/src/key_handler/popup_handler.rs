use super::{AppEventResult, KeyEventHandler};
use crate::entity::event::app_event::{AppEvent, PopupEvent};
use crossterm::event::{KeyCode, KeyEvent};

pub struct PopupHandler;

impl KeyEventHandler for PopupHandler {
    fn handle(&self, key_event: KeyEvent) -> AppEventResult {
        match key_event {
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
        }
    }
}
