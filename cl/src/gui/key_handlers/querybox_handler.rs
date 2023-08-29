use super::KeyEventHandler;
use crate::gui::entities::events::app_events::{AppEvent, QueryboxEvent};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub struct QueryboxHandler;

impl KeyEventHandler for QueryboxHandler {
    fn handle(&self, key_event: KeyEvent) -> Result<Option<AppEvent>> {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc | KeyCode::Enter | KeyCode::Down | KeyCode::Up,
                ..
            } => Ok(Some(AppEvent::QueryBox(QueryboxEvent::Deactive))),
            input => Ok(Some(AppEvent::QueryBox(QueryboxEvent::Input(input)))),
        }
    }
}
