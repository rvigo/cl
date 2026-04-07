use crate::component::{EditableTextbox, FutureEventType, List, Popup, Search, Tabs, TextBox};
use crate::observer::event::PopupType::{Dialog, Help};
use crate::observer::event::{Event, ListEvent, PopupEvent, SearchEvent, TabsEvent, TextBoxEvent};
use crate::screen::key_mapping::command::ScreenCommandCallback;
use crate::screen::key_mapping::ScreenCommand::{
    AddLayer, CopyToClipboard, Quit, ReplaceCurrentLayer,
};
use crate::screen::key_mapping::{KeyMapping, ScreenCommand};
use crate::screen::layer::{FormScreenLayer, MainScreenLayer, PopupLayer, QuickSearchLayer};
use crate::screen::ActiveScreen::Main;
use crate::state::selected_command::SelectedCommand;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{
    DeleteCommand, ExecuteCommand, GetCurrentQuery, NextTab, PreviousTab, SelectNextCommand,
    SelectPreviousCommand,
};
use crate::{async_fn_body, event, oneshot, run_if_some};
use anyhow::bail;
use async_trait::async_trait;
use cl_core::CommandVecExt;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::debug;
use std::any::TypeId;
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
                AddLayer(Box::new(PopupLayer::default())),
                event!(
                    Popup,
                    PopupEvent::Create(Dialog(
                        "Are you sure u want to delete this command?".to_string(),
                        FutureEventType::State(|state| {
                            async_fn_body! {
                                let result = oneshot!(state, DeleteCommand);
                                match result {
                                    Some((ok, reason)) => {
                                        if !ok {
                                            let msg = reason.unwrap_or_else(|| "unknown error".to_string());
                                            debug!("delete command failed: {}", msg);
                                            bail!(msg)
                                        } else {
                                            debug!("Command deleted");
                                            Ok(())
                                        }
                                    }
                                    None => {
                                        debug!("delete command: no response from state actor");
                                        Ok(())
                                    }
                                }
                            }
                        }),
                        ScreenCommandCallback::UpdateAll,
                    ))
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
                            event!(List, ListEvent::Next(cmd.current_idx)),
                            event!(TextBox, TextBoxEvent::UpdateCommand(cmd.value.clone())),
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
                            event!(List, ListEvent::Previous(cmd.current_idx)),
                            event!(TextBox, TextBoxEvent::UpdateCommand(cmd.value.clone())),
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
                        event!(List, ListEvent::UpdateAll(new_items.aliases())),
                        event!(Tabs, TabsEvent::Next(selected_namespace.idx)),
                        event!(
                            TextBox,
                            TextBoxEvent::UpdateCommand(selected_command.value.clone())
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
                        event!(List, ListEvent::UpdateAll(new_items.aliases())),
                        event!(Tabs, TabsEvent::Previous(selected_namespace.idx)),
                        event!(
                            TextBox,
                            TextBoxEvent::UpdateCommand(selected_command.value.clone())
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
                    AddLayer(Box::new(QuickSearchLayer::default())),
                    event!(Search, SearchEvent::UpdateQuery(current_query)),
                ];
                Some(events)
            }
            KeyEvent {
                code: KeyCode::Char('?'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(vec![
                AddLayer(Box::new(PopupLayer::default())),
                event!(Popup, PopupEvent::Create(Help(Main))),
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
            KeyEvent {
                code: KeyCode::Char('e'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                let events = vec![
                    ReplaceCurrentLayer(Box::new(FormScreenLayer::edit())),
                    ScreenCommand::Callback(ScreenCommandCallback::LoadCommandDetails(
                        TypeId::of::<EditableTextbox>(),
                    )),
                ];
                Some(events)
            }
            KeyEvent {
                code: KeyCode::Char('i'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                let events = vec![ReplaceCurrentLayer(Box::new(FormScreenLayer::insert()))];
                Some(events)
            }
            _ => None,
        }
    }
}
