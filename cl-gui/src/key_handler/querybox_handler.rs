use super::{AppEventResult, KeyEventHandler};
use crate::entity::event::app_event::{AppEvent, QueryboxEvent};
use crossterm::event::{KeyCode, KeyEvent};

pub struct QueryboxHandler;

impl KeyEventHandler for QueryboxHandler {
    fn handle(&self, key_event: KeyEvent) -> AppEventResult {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc | KeyCode::Enter | KeyCode::Down | KeyCode::Up,
                ..
            } => Ok(Some(AppEvent::QueryBox(QueryboxEvent::Deactive))),
            input => Ok(Some(AppEvent::QueryBox(QueryboxEvent::Input(input)))),
        }
    }
}
