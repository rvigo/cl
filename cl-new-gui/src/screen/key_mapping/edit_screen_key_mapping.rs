use crate::component::EditableTextbox;
use crate::event;
use crate::observer::event::{EditEvent, Event};
use crate::screen::key_mapping::command::EditCallback;
use crate::screen::key_mapping::{KeyMapping, ScreenCommand};
use crate::screen::layer::{EditScreenLayer, Layer, MainScreenLayer};
use crate::screen::ScreenCommandCallback::UpdateAll;
use crate::state::state_event::{FieldType, StateEvent};
use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::debug;
use tokio::sync::mpsc::Sender;

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
                let events = vec![
                    ScreenCommand::ReplaceCurrentLayer(Box::new(MainScreenLayer::new())),
                    ScreenCommand::Callback(UpdateAll),
                ];
                debug!(target: "clr_edit_screen_key_mapping", "{:?}", "Exiting edit screen");
                Some(events)
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
                // get the field name
                let current_field = self.get_next_field();
                let events = vec![
                    event!(
                        EditableTextbox,
                        Event::Edit(EditEvent::SetField(current_field.clone()))
                    ),
                    event!(FieldType, Event::Edit(EditEvent::SetField(current_field))),
                ];
                Some(events)
            }
            KeyEvent {
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                // get the field name
                let current_field = self.get_previous_field();
                let events = vec![
                    event!(
                        EditableTextbox,
                        Event::Edit(EditEvent::SetField(current_field.clone()))
                    ),
                    event!(FieldType, Event::Edit(EditEvent::SetField(current_field))),
                ];
                Some(events)
            }
            input => {
                debug!(target: "clr_edit_screen_key_mapping", "Received key event: {:?}", input);
                Some(vec![event!(EditableTextbox, Event::KeyEvent(input))])
            }
        }
    }
}
