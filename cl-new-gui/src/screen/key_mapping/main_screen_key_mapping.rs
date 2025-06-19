use crate::component::{List, Popup, Search, Tabs, TextBox};
use crate::observer::event::PopupType::{Dialog, Help};
use crate::observer::event::{Event, PopupEvent};
use crate::screen::key_mapping::ScreenCommand::{AddLayer, CopyToClipboard, Quit};
use crate::screen::key_mapping::{KeyMapping, ScreenCommand};
use crate::screen::layer::{Layer, MainScreenLayer, PopupLayer, QuickSearchLayer};
use crate::screen::ActiveScreen::Main;
use crate::state::selected_command::SelectedCommand;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{
    ExecuteCommand, GetCurrentQuery, NextTab, PreviousTab, SelectNextCommand, SelectPreviousCommand,
};
use crate::{event, oneshot, run_if_some};
use async_trait::async_trait;
use cl_core::CommandVecExt;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::debug;
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
                    run_if_some!(selected_command, |cmd: SelectedCommand| {
                        let events = vec![
                            event!(List, Event::Next(cmd.current_idx)),
                            event!(TextBox, Event::UpdateCommand(cmd.value.clone())),
                        ];

                        Some(events)
                    })
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
                    run_if_some!(selected_command, |cmd: SelectedCommand| {
                        let events = vec![
                            event!(List, Event::Previous(cmd.current_idx)),
                            event!(TextBox, Event::UpdateCommand(cmd.value.clone())),
                        ];

                        Some(events)
                    })
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
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('/'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                debug!("getting current query from state");
                let current_query = oneshot!(state_tx, GetCurrentQuery).unwrap_or_default();

                let events = vec![
                    AddLayer(Box::new(QuickSearchLayer::new())),
                    event!(Search, Event::UpdateQuery(current_query)),
                ];
                Some(events)
            }
            KeyEvent {
                code: KeyCode::Char('?'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(vec![
                AddLayer(Box::new(PopupLayer::new())),
                event!(Popup, Event::Popup(PopupEvent::Create(Help(Main)))),
            ]),
            KeyEvent {
                code: KeyCode::Char('y'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(vec![CopyToClipboard]),
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
