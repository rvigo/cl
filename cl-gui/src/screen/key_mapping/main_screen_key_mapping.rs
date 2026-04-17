use crate::component::{EditableTextbox, FutureEventType, List, Popup, Search, Tabs, TextBox};
use crate::observer::event::PopupType::{Dialog, Help};
use crate::observer::event::{ListEvent, PopupEvent, SearchEvent, TabsEvent, TextBoxEvent};
use crate::screen::key_mapping::command::ScreenCommandCallback;
use crate::screen::key_mapping::ScreenCommand::{
    AddLayer, CopyToClipboard, Quit, ReplaceCurrentLayer,
};
use crate::screen::key_mapping::{create_notify_command, ScreenCommand};
use crate::screen::layer::{FormScreenLayer, MainScreenLayer, PopupLayer, QuickSearchLayer};
use crate::screen::ActiveScreen::Main;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{
    DeleteCommand, ExecuteCommand, GetCurrentQuery, NextTab, PreviousTab,
};
use cl_core::CommandVecExt;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::any::TypeId;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::Sender;
use tracing::debug;

impl MainScreenLayer {
    pub(crate) fn map_key_event<'a>(
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
                        "Are you sure you want to delete this command?".to_string(),
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
                // Navigate next — handled by CommandDispatcher using the local snapshot
                KeyEvent {
                    code: KeyCode::Char('j'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => Some(vec![ScreenCommand::NavigateNext]),
                // Navigate previous — handled by CommandDispatcher using the local snapshot
                KeyEvent {
                    code: KeyCode::Char('k'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => Some(vec![ScreenCommand::NavigatePrev]),
                KeyEvent {
                    code: KeyCode::Char('l'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => match oneshot!(state_tx, NextTab) {
                    Ok((selected_namespace, selected_command, new_items)) => {
                        let aliases = new_items.aliases();
                        let events = vec![
                            ScreenCommand::SetSnapshot {
                                items: new_items,
                                selected_idx: 0,
                            },
                            create_notify_command::<List>(ListEvent::UpdateAll(aliases)),
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
                        let aliases = new_items.aliases();
                        let events = vec![
                            ScreenCommand::SetSnapshot {
                                items: new_items,
                                selected_idx: 0,
                            },
                            create_notify_command::<List>(ListEvent::UpdateAll(aliases)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screen::layer::Layer;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    fn run_key(code: KeyCode) -> Option<Vec<ScreenCommand>> {
        let layer = MainScreenLayer::default();
        let (tx, _rx) = tokio::sync::mpsc::channel(16);
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(layer.handle_key_event(key(code), tx))
    }

    #[test]
    fn quit_key_returns_quit_command() {
        let result = run_key(KeyCode::Char('q'));
        assert!(result.is_some());
        let cmds = result.unwrap();
        assert!(
            cmds.iter().any(|c| matches!(c, ScreenCommand::Quit)),
            "expected Quit command"
        );
    }

    #[test]
    fn copy_key_returns_copy_command() {
        let result = run_key(KeyCode::Char('y'));
        assert!(result.is_some());
        let cmds = result.unwrap();
        assert!(
            cmds.iter()
                .any(|c| matches!(c, ScreenCommand::CopyToClipboard)),
            "expected CopyToClipboard command"
        );
    }

    #[test]
    fn j_key_returns_navigate_next() {
        let result = run_key(KeyCode::Char('j'));
        assert!(result.is_some());
        let cmds = result.unwrap();
        assert!(
            cmds.iter()
                .any(|c| matches!(c, ScreenCommand::NavigateNext)),
            "expected NavigateNext command"
        );
    }

    #[test]
    fn k_key_returns_navigate_prev() {
        let result = run_key(KeyCode::Char('k'));
        assert!(result.is_some());
        let cmds = result.unwrap();
        assert!(
            cmds.iter()
                .any(|c| matches!(c, ScreenCommand::NavigatePrev)),
            "expected NavigatePrev command"
        );
    }

    #[test]
    fn edit_key_returns_replace_layer() {
        let result = run_key(KeyCode::Char('e'));
        assert!(result.is_some());
        let cmds = result.unwrap();
        assert!(
            cmds.iter()
                .any(|c| matches!(c, ScreenCommand::ReplaceCurrentLayer(_))),
            "expected ReplaceCurrentLayer for edit"
        );
    }

    #[test]
    fn insert_key_returns_replace_layer() {
        let result = run_key(KeyCode::Char('i'));
        assert!(result.is_some());
        let cmds = result.unwrap();
        assert!(
            cmds.iter()
                .any(|c| matches!(c, ScreenCommand::ReplaceCurrentLayer(_))),
            "expected ReplaceCurrentLayer for insert"
        );
    }

    #[test]
    fn unknown_key_returns_none() {
        let result = run_key(KeyCode::Char('z'));
        assert!(result.is_none());
    }
}
