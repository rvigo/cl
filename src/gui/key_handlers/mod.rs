mod edit_handler;
pub mod input_handler;
mod insert_handler;
mod main_handler;
mod popup_handler;
mod query_box_handler;

use super::entities::{events::app_events::AppEvent, ui_context::UIContext};
use anyhow::Result;
use crossterm::event::KeyEvent;

pub trait KeyEventHandler {
    fn handle(&self, key_event: KeyEvent) -> Result<Option<AppEvent>>;
}

pub trait WidgetKeyEventHandler {
    fn handle(&self, key_event: KeyEvent, ui_context: &mut UIContext) -> Result<Option<AppEvent>>;
}
