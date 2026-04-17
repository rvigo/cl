use crate::component::{Downcastable, EditableTextbox, Popup};
use crate::component::{FutureEventType, ScreenState};
use crate::observer::event::PopupType::{Dialog, Help};
use crate::observer::event::{EditableTextboxEvent, PopupEvent, ScreenStateEvent};
use crate::screen::command::ScreenCommand::AddLayer;
use crate::screen::command::ScreenCommandCallback;
use crate::screen::command::ScreenCommandCallback::UpdateAll;
use crate::screen::key_mapping::command::FormCallback;
use crate::screen::key_mapping::{create_notify_command, ScreenCommand};
use crate::screen::layer::{FormScreenLayer, MainScreenLayer, PopupLayer};
use crate::screen::ActiveScreen;
use crate::state::state_event::StateEvent;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::Sender;
use tracing::debug;

impl FormScreenLayer {
    pub(crate) fn map_key_event<'a>(
        &'a self,
        key: KeyEvent,
        _: Sender<StateEvent>,
    ) -> Pin<Box<dyn Future<Output = Option<Vec<ScreenCommand>>> + 'a>> {
        // Extract all self-borrowed data up front so the async block owns
        // only plain values — no borrow of `self` crosses the await point.
        let has_changes = self
            .screen_state
            .as_observable()
            .downcast_to::<ScreenState>()
            .is_some_and(|s| s.has_changes);
        let next_field = self.get_next_field();
        let prev_field = self.get_previous_field();
        let mode = self.mode;

        Box::pin(async move {
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
                    if has_changes {
                        debug!(target: "clr_form_screen_key_mapping", "Has unsaved changes, asking for confirmation");
                        Some(vec![
                            AddLayer(Box::new(PopupLayer::default())),
                            create_notify_command::<Popup>(PopupEvent::Create(Dialog(
                                "You have unsaved changes. Exit anyway?".to_string(),
                                FutureEventType::State(|_| async_fn_body! { Ok(()) }),
                                ScreenCommandCallback::ExitEditScreen,
                            ))),
                        ])
                    } else {
                        debug!(target: "clr_form_screen_key_mapping", "Exiting form screen");
                        Some(vec![
                            ScreenCommand::ReplaceCurrentLayer(
                                Box::new(MainScreenLayer::default()),
                            ),
                            ScreenCommand::Callback(UpdateAll),
                        ])
                    }
                }
                KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                } => {
                    let events = vec![
                        ScreenCommand::GetFieldContent,
                        ScreenCommand::Form(FormCallback::Save(mode)),
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
                    let events = vec![
                        create_notify_command::<EditableTextbox>(EditableTextboxEvent::SetField(
                            next_field,
                        )),
                        create_notify_command::<ScreenState>(ScreenStateEvent::SetField(
                            next_field,
                        )),
                    ];
                    Some(events)
                }
                KeyEvent {
                    code: KeyCode::BackTab,
                    modifiers: KeyModifiers::SHIFT,
                    ..
                } => {
                    let events = vec![
                        create_notify_command::<EditableTextbox>(EditableTextboxEvent::SetField(
                            prev_field,
                        )),
                        create_notify_command::<ScreenState>(ScreenStateEvent::SetField(
                            prev_field,
                        )),
                    ];
                    Some(events)
                }
                KeyEvent {
                    code: KeyCode::F(1),
                    ..
                } => Some(vec![
                    AddLayer(Box::new(PopupLayer::default())),
                    create_notify_command::<Popup>(PopupEvent::Create(Help(ActiveScreen::Form))),
                ]),
                input => {
                    debug!(target: "clr_form_screen_key_mapping", "Received key event: {:?}", input);
                    Some(vec![
                        create_notify_command::<EditableTextbox>(EditableTextboxEvent::KeyInput(
                            input,
                        )),
                        create_notify_command::<ScreenState>(ScreenStateEvent::KeyInput(input)),
                    ])
                }
            }
        })
    }
}
