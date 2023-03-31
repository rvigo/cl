use crate::gui::entities::{
    events::app_events::{AppEvents, QueryboxEvent},
    ui_context::UIContext,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use parking_lot::Mutex;
use std::sync::Arc;

pub fn handle(
    key_event: KeyEvent,
    ui_context: &mut Arc<Mutex<UIContext<'static>>>,
) -> Result<Option<AppEvents>> {
    match key_event {
        KeyEvent {
            code: KeyCode::Esc | KeyCode::Enter | KeyCode::Down | KeyCode::Up,
            ..
        } => return Ok(Some(AppEvents::QueryBox(QueryboxEvent::Deactive))),
        input => ui_context.lock().handle_querybox_input(input),
    }

    Ok(None)
}
