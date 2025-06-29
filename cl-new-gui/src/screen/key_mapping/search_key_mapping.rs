use crate::component::Search;
use crate::observer::event::Event;
use crate::observer::event::Event::UpdateContent;
use crate::observer::event::SearchAction::Input;
use crate::screen::key_mapping::ScreenCommand::Notify;
use crate::screen::key_mapping::{KeyMapping, ScreenCommand};
use crate::screen::layer::QuickSearchLayer;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::GetCurrentQuery;
use crate::{event, oneshot};
use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent};
use std::any::TypeId;
use tokio::sync::mpsc::Sender;
use crate::screen::key_mapping::command::ScreenCommandCallback::UpdateAll;

#[async_trait(?Send)]
impl KeyMapping for QuickSearchLayer {
    async fn handle_key_event(
        &self,
        key: KeyEvent,
        state_tx: Sender<StateEvent>,
    ) -> Option<Vec<ScreenCommand>> {
        match key {
            KeyEvent {
                code: KeyCode::Esc | KeyCode::Enter | KeyCode::Down | KeyCode::Up,
                ..
            } => {
                let mut events = vec![ScreenCommand::PopLastLayer(None)];
                let result = oneshot!(state_tx, GetCurrentQuery);
                if let Some(query) = result {
                    events.push(Notify((
                        TypeId::of::<Search>(),
                        UpdateContent(query),
                    )))
                }

                Some(events)
            }
            _ => Some(vec![
                event!(Search, Event::Search(Input(key,), state_tx)),
                ScreenCommand::Callback(UpdateAll),
            ]),
        }
    }
}
