pub(super) mod edit_handler;
pub(super) mod help_popup_handler;
pub(super) mod insert_handler;
pub(super) mod main_handler;
pub(super) mod popup_handler;
pub(super) mod querybox_handler;

use super::entities::events::app_events::AppEvent;
use anyhow::Result;
use crossterm::event::KeyEvent;

type AppEventResult = Result<Option<AppEvent>>;

/// (Almost) Every KeyEvent triggers an app event
pub trait KeyEventHandler {
    fn handle(&self, key_event: KeyEvent) -> AppEventResult;
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum HandlerType {
    Main,
    Insert,
    Edit,
    Popup,
    Help,
    QueryBox,
}
