use super::KeyEventHandler;
use crate::gui::entities::events::app_events::{AppEvent, PopupEvent::Disable};
use anyhow::Result;
use crossterm::event::KeyEvent;

pub struct HelpPopupHandler;

impl KeyEventHandler for HelpPopupHandler {
    fn handle(&self, _: KeyEvent) -> Result<Option<AppEvent>> {
        Ok(Some(AppEvent::Popup(Disable)))
    }
}
