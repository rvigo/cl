use crate::component::{Downcastable, EditableTextbox, Popup};
use crate::component::{FutureEventType, ScreenState};
use crate::observer::event::PopupType::Dialog;
use crate::observer::event::{EditableTextboxEvent, PopupEvent, ScreenStateEvent};
use crate::screen::command::ScreenCommand::AddLayer;
use crate::screen::command::ScreenCommandCallback;
use crate::screen::key_mapping::command::FormCallback;
use crate::screen::key_mapping::{notify, KeyMapping, ScreenCommand};
use crate::screen::layer::{FormScreenLayer, MainScreenLayer, PopupLayer};
use crate::screen::ScreenCommandCallback::UpdateAll;
use crate::state::state_event::StateEvent;
use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tracing::debug;
use tokio::sync::mpsc::Sender;

#[async_trait(?Send)]
impl KeyMapping for FormScreenLayer {
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
                if let Some(inner) =
                    self.screen_state.as_observable().downcast_to::<ScreenState>()
                {
                    if inner.has_changes {
                        debug!(target: "clr_form_screen_key_mapping", "Has unsaved changes, asking for confirmation");
                        Some(vec![
                            AddLayer(Box::new(PopupLayer::default())),
                            notify::<Popup>(PopupEvent::Create(Dialog(
                                "You have unsaved changes. Exit anyway?".to_string(),
                                FutureEventType::State(|_| async_fn_body! { Ok(()) }),
                                ScreenCommandCallback::ExitEditScreen,
                            ))),
                        ])
                    } else {
                        debug!(target: "clr_form_screen_key_mapping", "Exiting form screen");
                        Some(vec![
                            ScreenCommand::ReplaceCurrentLayer(Box::new(
                                MainScreenLayer::default(),
                            )),
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
                    ScreenCommand::Form(FormCallback::Save(self.mode)),
                    ScreenCommand::ReplaceCurrentLayer(Box::new(MainScreenLayer::default())),
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
                    notify::<EditableTextbox>(EditableTextboxEvent::SetField(current_field.clone())),
                    notify::<ScreenState>(ScreenStateEvent::SetField(current_field)),
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
                    notify::<EditableTextbox>(EditableTextboxEvent::SetField(current_field.clone())),
                    notify::<ScreenState>(ScreenStateEvent::SetField(current_field)),
                ];
                Some(events)
            }
            input => {
                debug!(target: "clr_form_screen_key_mapping", "Received key event: {:?}", input);
                Some(vec![
                    notify::<EditableTextbox>(EditableTextboxEvent::KeyInput(input)),
                    notify::<ScreenState>(ScreenStateEvent::KeyInput(input)),
                ])
            }
        }
    }
}
