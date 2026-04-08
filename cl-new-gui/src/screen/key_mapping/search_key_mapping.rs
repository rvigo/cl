use crate::component::Search;
use crate::observer::event::SearchEvent;
use crate::screen::key_mapping::{create_notify_command, KeyMapping, ScreenCommand};
use crate::screen::layer::QuickSearchLayer;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::GetCurrentQuery;
use async_trait::async_trait;
use crossterm::event::{KeyCode, KeyEvent};
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
                if let Ok(query) = result {
                    events.push(create_notify_command::<Search>(SearchEvent::UpdateQuery(query)));
                }

                Some(events)
            }
            _ => Some(vec![
                create_notify_command::<Search>(SearchEvent::Input(key, state_tx)),
                ScreenCommand::Callback(UpdateAll),
            ]),
        }
    }
}
