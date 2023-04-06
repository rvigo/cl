use super::WidgetKeyEventHandler;
use crate::gui::entities::{
    events::app_events::{AppEvent, QueryboxEvent},
    ui_context::UIContext,
};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

pub struct QueryboxHandler;

impl WidgetKeyEventHandler for QueryboxHandler {
    fn handle(&self, key_event: KeyEvent, ui_context: &mut UIContext) -> Result<Option<AppEvent>> {
        match key_event {
            KeyEvent {
                code: KeyCode::Esc | KeyCode::Enter | KeyCode::Down | KeyCode::Up,
                ..
            } => return Ok(Some(AppEvent::QueryBox(QueryboxEvent::Deactive))),
            input => ui_context.handle_querybox_input(input),
        }

        Ok(None)
    }
}
