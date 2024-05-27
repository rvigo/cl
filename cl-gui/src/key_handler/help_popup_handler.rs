use super::{AppEventResult, KeyEventHandler};
use crate::entity::event::app_event::{AppEvent, PopupEvent::Disable};
use crossterm::event::KeyEvent;

pub struct HelpPopupHandler;

impl KeyEventHandler for HelpPopupHandler {
    fn handle(&self, _: KeyEvent) -> AppEventResult {
        Ok(Some(AppEvent::Popup(Disable)))
    }
}
