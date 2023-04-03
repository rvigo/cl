use super::WidgetKeyEventHandler;
use crate::gui::entities::{
    events::app_events::{AppEvents, QueryboxEvent},
    ui_context::UIContext,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub struct QueryboxHandler;

impl WidgetKeyEventHandler for QueryboxHandler {
    fn handle(&self, key_event: KeyEvent, ui_context: &mut UIContext) -> Result<Option<AppEvents>> {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc | KeyCode::Enter | KeyCode::Down | KeyCode::Up,
                ..
            } => return Ok(Some(AppEvents::QueryBox(QueryboxEvent::Deactive))),
            input => ui_context.handle_querybox_input(input),
        }

        Ok(None)
    }
}
