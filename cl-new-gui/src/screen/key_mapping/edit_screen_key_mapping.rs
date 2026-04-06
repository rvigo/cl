use crate::component::{Downcastable, EditableTextbox, Popup};
use crate::component::{FutureEventType, ScreenState};
use crate::event;
use crate::observer::event::PopupType::Dialog;
use crate::observer::event::{EditEvent, Event, PopupEvent};
use crate::screen::command::ScreenCommand::AddLayer;
use crate::screen::command::ScreenCommandCallback;
use crate::screen::key_mapping::command::EditCallback;
use crate::screen::key_mapping::{KeyMapping, ScreenCommand};
use crate::screen::layer::{EditScreenLayer, Layer, MainScreenLayer, PopupLayer};
use crate::screen::ScreenCommandCallback::UpdateAll;
use crate::state::state_event::StateEvent;
use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::debug;
use tokio::sync::mpsc::Sender;
use crate::async_fn_body;

#[async_trait(?Send)]
impl KeyMapping for EditScreenLayer {
    async fn handle_key_event(
        &self,
        key: KeyEvent,
        _: Sender<StateEvent>,
    ) -> Option<Vec<ScreenCommand>> {
        match key {
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                if let Some(inner) = self.screen_state.borrow().downcast_to::<ScreenState>() {
                    if inner.has_changes {
                        debug!(target: "clr_edit_screen_key_mapping", "Has unsaved changes, asking for confirmation");
                        Some(vec![
                            AddLayer(Box::new(PopupLayer::new())),
                            event!(
                                Popup,
                                Event::Popup(PopupEvent::Create(Dialog(
                                    "You have unsaved changes. Exit anyway?".to_string(),
                                    FutureEventType::State(|_| async_fn_body! { Ok(()) }),
                                    ScreenCommandCallback::ExitEditScreen,
                                )))
                            ),
                        ])
                    } else {
                        debug!(target: "clr_edit_screen_key_mapping", "Exiting edit screen");
                        Some(vec![
                            ScreenCommand::ReplaceCurrentLayer(Box::new(MainScreenLayer::new())),
                            ScreenCommand::Callback(UpdateAll),
                        ])
                    }
                } else {
                    None
                }
            }
            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                let events = vec![
                    ScreenCommand::GetFieldContent,
                    ScreenCommand::Edit(EditCallback::Save),
                    ScreenCommand::ReplaceCurrentLayer(Box::new(MainScreenLayer::new())),
                    ScreenCommand::Callback(UpdateAll),
                ];
                Some(events)
            }
            KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                let current_field = self.get_next_field();
                let events = vec![
                    event!(
                        EditableTextbox,
                        Event::Edit(EditEvent::SetField(current_field.clone()))
                    ),
                    event!(ScreenState, Event::Edit(EditEvent::SetField(current_field))),
                ];
                Some(events)
            }
            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                let current_field = self.get_previous_field();
                let events = vec![
                    event!(
                        EditableTextbox,
                        Event::Edit(EditEvent::SetField(current_field.clone()))
                    ),
                    event!(ScreenState, Event::Edit(EditEvent::SetField(current_field))),
                ];
                Some(events)
            }
            input => {
                debug!(target: "clr_edit_screen_key_mapping", "Received key event: {:?}", input);
                Some(vec![
                    event!(EditableTextbox, Event::KeyEvent(input)),
                    event!(ScreenState, Event::KeyEvent(input)),
                ])
            }
        }
    }
}
