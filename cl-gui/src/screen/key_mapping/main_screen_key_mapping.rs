use crate::component::{EditableTextbox, FutureEventType, List, Popup, Search, Tabs, TextBox};
use crate::observer::event::PopupType::{Dialog, Help};
use crate::observer::event::{ListEvent, PopupEvent, SearchEvent, TabsEvent, TextBoxEvent};
use crate::screen::key_mapping::command::ScreenCommandCallback;
use crate::screen::key_mapping::ScreenCommand::{
    AddLayer, CopyToClipboard, Quit, ReplaceCurrentLayer,
};
use crate::screen::key_mapping::{create_notify_command, KeyMapping, ScreenCommand};
use crate::screen::layer::{FormScreenLayer, MainScreenLayer, PopupLayer, QuickSearchLayer};
use crate::screen::ActiveScreen::Main;
use crate::state::selected_command::SelectedCommand;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{
    DeleteCommand, ExecuteCommand, GetCurrentQuery, NextTab, PreviousTab, SelectNextCommand,
    SelectPreviousCommand,
};
use cl_core::CommandVecExt;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::any::TypeId;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::Sender;
use tracing::debug;

impl KeyMapping for MainScreenLayer {
    fn handle_key_event<'a>(
        &'a self,
        key: KeyEvent,
        state_tx: Sender<StateEvent>,
    ) -> Pin<Box<dyn Future<Output = Option<Vec<ScreenCommand>>> + 'a>> {
        Box::pin(async move {
            match key {
                KeyEvent {
                    code: KeyCode::Char('d'),
                    ..
                } => Some(vec![
                    AddLayer(Box::new(PopupLayer::default())),
                    create_notify_command::<Popup>(PopupEvent::Create(Dialog(
                        "Are you sure u want to delete this command?".to_string(),
                        FutureEventType::State(|state| {
                            async_fn_body! {
                                let result = oneshot!(state, DeleteCommand)?;
                                result.map_err(|e| anyhow::anyhow!(e))?;
                                debug!("Command deleted");
                                Ok(())
                            }
                        }),
                        ScreenCommandCallback::UpdateAll,
                    ))),
                ]),
                KeyEvent {
                    code: KeyCode::Char('j'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    if let Ok(selected_command) = oneshot!(state_tx, SelectNextCommand) {
                        selected_command.map(|cmd: SelectedCommand| {
                            vec![
                                create_notify_command::<List>(ListEvent::Next(cmd.current_idx)),
                                create_notify_command::<TextBox>(TextBoxEvent::UpdateCommand(
                                    cmd.value.clone(),
                                )),
                            ]
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
                    if let Ok(selected_command) = oneshot!(state_tx, SelectPreviousCommand) {
                        selected_command.map(|cmd: SelectedCommand| {
                            vec![
                                create_notify_command::<List>(ListEvent::Previous(cmd.current_idx)),
                                create_notify_command::<TextBox>(TextBoxEvent::UpdateCommand(
                                    cmd.value.clone(),
                                )),
                            ]
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
                    Ok((selected_namespace, selected_command, new_items)) => {
                        let events = vec![
                            create_notify_command::<List>(ListEvent::UpdateAll(
                                new_items.aliases(),
                            )),
                            create_notify_command::<Tabs>(TabsEvent::Next(selected_namespace.idx)),
                            create_notify_command::<TextBox>(TextBoxEvent::UpdateCommand(
                                selected_command.value.clone(),
                            )),
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
                    Ok((selected_namespace, selected_command, new_items)) => {
                        let events = vec![
                            create_notify_command::<List>(ListEvent::UpdateAll(
                                new_items.aliases(),
                            )),
                            create_notify_command::<Tabs>(TabsEvent::Previous(
                                selected_namespace.idx,
                            )),
                            create_notify_command::<TextBox>(TextBoxEvent::UpdateCommand(
                                selected_command.value.clone(),
                            )),
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
                        create_notify_command::<Search>(SearchEvent::UpdateQuery(current_query)),
                    ];
                    Some(events)
                }
                KeyEvent {
                    code: KeyCode::F(1),
                    ..
                }
                | KeyEvent {
                    code: KeyCode::Char('?'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => Some(vec![
                    AddLayer(Box::new(PopupLayer::default())),
                    create_notify_command::<Popup>(PopupEvent::Create(Help(Main))),
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
                    if let Err(e) = state_tx.send(ExecuteCommand).await {
                        tracing::error!("failed to send ExecuteCommand: {e}");
                    }
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
        })
    }
}
