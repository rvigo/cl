use crate::component::Popup;
use crate::event;
use crate::observer::event::{Event, PopupEvent};
use crate::screen::key_mapping::{KeyMapping, ScreenCommand};
use crate::screen::layer::PopupLayer;
use crate::state::state_event::StateEvent;
use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tokio::sync::mpsc::Sender;

#[async_trait(?Send)]
impl KeyMapping for PopupLayer {
    async fn handle_key_event(
        &self,
        key: KeyEvent,
        state_tx: Sender<StateEvent>,
    ) -> Option<Vec<ScreenCommand>> {
        match key {
            KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                let event = event!(Popup, Event::Popup(PopupEvent::PreviousChoice));
                Some(vec![event])
            }
            KeyEvent {
                code: KeyCode::Char('l'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                let event = event!(Popup, Event::Popup(PopupEvent::NextChoice));
                Some(vec![event])
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                let event = event!(Popup, Event::Popup(PopupEvent::Run(state_tx)));
                Some(vec![event, ScreenCommand::PopLastLayer])
            }
            _ => None,
        }
    }
}
