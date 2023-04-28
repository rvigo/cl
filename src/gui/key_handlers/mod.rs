pub(super) mod edit_handler;
pub(super) mod help_popup_handler;
pub(super) mod insert_handler;
pub(super) mod main_handler;
pub(super) mod popup_handler;
pub(super) mod querybox_handler;

use super::entities::events::app_events::AppEvent;
use anyhow::Result;
use crossterm::event::KeyEvent;

/// (Almost) Every KeyEvent triggers an app event
pub trait KeyEventHandler {
    fn handle(&self, key_event: KeyEvent) -> Result<Option<AppEvent>>;
}
