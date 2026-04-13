use crate::component::Popup;
use crate::observer::event::PopupEvent;
use crate::screen::key_mapping::ScreenCommand::PopLastLayer;
use crate::screen::key_mapping::{create_notify_command, KeyMapping, ScreenCommand};
use crate::screen::layer::PopupLayer;
use crate::state::state_event::StateEvent;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::Sender;

impl KeyMapping for PopupLayer {
    fn handle_key_event<'a>(
        &'a self,
        key: KeyEvent,
        state_tx: Sender<StateEvent>,
    ) -> Pin<Box<dyn Future<Output = Option<Vec<ScreenCommand>>> + 'a>> {
        Box::pin(async move {
            match key {
                KeyEvent {
                    code: KeyCode::Char('h'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    let event = create_notify_command::<Popup>(PopupEvent::PreviousChoice);
                    Some(vec![event])
                }
                KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    let event = create_notify_command::<Popup>(PopupEvent::NextChoice);
                    Some(vec![event])
                }
                KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    // cannot use an actual oneshot here, but is the same idea
                    let (tx, rx) = tokio::sync::mpsc::channel(1);
                    let event =
                        create_notify_command::<Popup>(PopupEvent::Run(state_tx.clone(), tx));

                    Some(vec![event, PopLastLayer(Some(rx))])
                }
                _ => Some(vec![PopLastLayer(None)]),
            }
        })
    }
}
