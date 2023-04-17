pub mod edit_handler;
pub mod help_popup_handler;
pub mod insert_handler;
pub mod main_handler;
pub mod popup_handler;
pub mod querybox_handler;

use super::entities::{contexts::ui_context::UIContext, events::app_events::AppEvent};
use anyhow::Result;
use crossterm::event::KeyEvent;

/// (Almost) Every KeyEvent triggers an app event
pub trait KeyEventHandler {
    fn handle(&self, key_event: KeyEvent) -> Result<Option<AppEvent>>;
}

pub trait WidgetKeyEventHandler {
    fn handle(&self, key_event: KeyEvent, ui_context: &mut UIContext) -> Result<Option<AppEvent>>;
}
