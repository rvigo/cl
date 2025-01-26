use crate::component::{List, Popup, Tabs, TextBox};
use crate::observer::event::PopupType::{Dialog, Help};
use crate::observer::event::{Event, PopupEvent};
use crate::screen::key_mapping::ScreenCommand::{AddLayer, Quit};
use crate::screen::key_mapping::{KeyMapping, ScreenCommand};
use crate::screen::layer::{Layer, MainScreenLayer, PopupLayer};
use crate::screen::ActiveScreen::Main;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{
    ExecuteCommand, NextTab, PreviousTab, SelectNextCommand, SelectPreviousCommand,
};
use crate::ui::ui_actor::CommandVecExt;
use crate::{event, oneshot};
use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tokio::sync::mpsc::Sender;

#[async_trait(?Send)]
impl KeyMapping for MainScreenLayer {
    async fn handle_key_event(
        &self,
        key: KeyEvent,
        state_tx: Sender<StateEvent>,
    ) -> Option<Vec<ScreenCommand>> {
        match key {
            KeyEvent {
                code: KeyCode::Char('d'),
                ..
            } => Some(vec![
                AddLayer(Box::new(PopupLayer::new())),
                event!(
                    Popup,
                    Event::Popup(PopupEvent::Create(Dialog(
                        "Are you sure u want to delete this command?".to_string()
                    )))
                ),
            ]),
            KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if let Some(selected_command) = oneshot!(state_tx, SelectNextCommand) {
                    let events = vec![
                        event!(List, Event::Next(selected_command.current_idx)),
                        event!(
                            TextBox,
                            Event::UpdateCommand(selected_command.value.clone())
                        ),
                    ];

                    Some(events)
                } else {
                    None
                }
            }
            KeyEvent {
                code: KeyCode::Char('k'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if let Some(selected_command) = oneshot!(state_tx, SelectPreviousCommand) {
                    let events = vec![
                        event!(List, Event::Previous(selected_command.current_idx)),
                        event!(
                            TextBox,
                            Event::UpdateCommand(selected_command.value.clone())
                        ),
                    ];

                    Some(events)
                } else {
                    None
                }
            }
            KeyEvent {
                code: KeyCode::Char('l'),
                modifiers: KeyModifiers::NONE,
                ..
            } => match oneshot!(state_tx, NextTab) {
                Some((selected_namespace, selected_command, new_items)) => {
                    let events = vec![
                        event!(List, Event::UpdateAll(new_items.aliases())),
                        event!(Tabs, Event::Next(selected_namespace.idx)),
                        event!(
                            TextBox,
                            Event::UpdateCommand(selected_command.value.clone())
                        ),
                    ];

                    Some(events)
                }
                _ => None,
            },
            KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
                ..
            } => match oneshot!(state_tx, PreviousTab) {
                Some((selected_namespace, selected_command, new_items)) => {
                    let events = vec![
                        event!(List, Event::UpdateAll(new_items.aliases())),
                        event!(Tabs, Event::Previous(selected_namespace.idx)),
                        event!(
                            TextBox,
                            Event::UpdateCommand(selected_command.value.clone())
                        ),
                    ];

                    Some(events)
                }
                _ => None,
            },

            KeyEvent {
                code: KeyCode::Char('?'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(vec![
                AddLayer(Box::new(PopupLayer::new())),
                event!(Popup, Event::Popup(PopupEvent::Create(Help(Main)))),
            ]),
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                state_tx.send(ExecuteCommand).await.ok();
                Some(vec![Quit])
            }
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(vec![Quit]),
            _ => None,
        }
    }
}
