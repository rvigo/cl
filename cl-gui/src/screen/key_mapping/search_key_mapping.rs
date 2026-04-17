use crate::component::Search;
use crate::observer::event::SearchEvent;
use crate::screen::key_mapping::command::ScreenCommandCallback::UpdateAll;
use crate::screen::key_mapping::{create_notify_command, ScreenCommand};
use crate::screen::layer::QuickSearchLayer;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::GetCurrentQuery;
use crossterm::event::{KeyCode, KeyEvent};
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::Sender;

impl QuickSearchLayer {
    pub(crate) fn map_key_event<'a>(
        &'a self,
        key: KeyEvent,
        state_tx: Sender<StateEvent>,
    ) -> Pin<Box<dyn Future<Output = Option<Vec<ScreenCommand>>> + 'a>> {
        Box::pin(async move {
            match key {
                KeyEvent {
                    code: KeyCode::Esc | KeyCode::Enter | KeyCode::Down | KeyCode::Up,
                    ..
                } => {
                    let mut events = vec![ScreenCommand::PopLastLayer(None)];
                    let result = oneshot!(state_tx, GetCurrentQuery);
                    if let Ok(query) = result {
                        events.push(create_notify_command::<Search>(SearchEvent::UpdateQuery(
                            query,
                        )));
                    }

                    Some(events)
                }
                _ => Some(vec![
                    create_notify_command::<Search>(SearchEvent::Input(key, state_tx)),
                    ScreenCommand::Callback(UpdateAll),
                ]),
            }
        })
    }
}
