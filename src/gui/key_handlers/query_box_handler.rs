use crate::gui::entities::{
    application_context::ApplicationContext,
    events::app_events::{AppEvents, QueryboxEvent},
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};
use parking_lot::Mutex;
use std::sync::Arc;

pub fn handle(
    key_event: KeyEvent,
    context: &mut Arc<Mutex<ApplicationContext>>,
) -> Result<Option<AppEvents>> {
    match key_event {
        KeyEvent {
            code: KeyCode::Esc | KeyCode::Enter | KeyCode::Down | KeyCode::Up,
            ..
        } => return Ok(Some(AppEvents::QueryBox(QueryboxEvent::Deactive))),
        input => context.lock().handle_querybox_input(input),
    }

    Ok(None)
}
