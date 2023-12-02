use super::{AppEventResult, KeyEventHandler};
use crate::entities::events::app_events::{AppEvent, PopupEvent::Disable};
use crossterm::event::KeyEvent;

pub struct HelpPopupHandler;

impl KeyEventHandler for HelpPopupHandler {
    fn handle(&self, _: KeyEvent) -> AppEventResult {
        Ok(Some(AppEvent::Popup(Disable)))
    }
}
