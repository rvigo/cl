use super::{AppEventResult, KeyEventHandler};
use crate::event::{AppEvent, PopupEvent};
use crossterm::event::KeyEvent;

pub struct HelpPopupHandler;

impl KeyEventHandler for HelpPopupHandler {
    fn handle(&self, _: KeyEvent) -> AppEventResult {
        Ok(Some(AppEvent::Popup(PopupEvent::Disable)))
    }
}
