use std::any::TypeId;
use log::debug;
use tokio::sync::mpsc::Sender;
use cl_core::{Command, CommandVecExt};
use crate::component::{List, Tabs, TextBox};
use crate::observer::event::Event;
use crate::oneshot;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{CommandDetails, CurrentCommand, GetAllListItems, GetAllNamespaces};

/// Enables the communication between the upper layer and the lower layer via callback commands
pub enum ScreenCommandCallback {
    /// Refreshes all info
    UpdateAll,
    /// Load command details
    LoadCommandDetails(TypeId),
    /// Save changes to the current command
    SaveChanges(Option<Command<'static>>),
    /// Do nothing
    DoNothing,
}

impl ScreenCommandCallback {
    pub async fn handle(self, state_tx: &Sender<StateEvent>) -> Option<Vec<(TypeId, Event)>> {
        match self {
            ScreenCommandCallback::UpdateAll => {
                let items = oneshot!(state_tx, GetAllListItems);
                let tabs = oneshot!(state_tx, GetAllNamespaces);
                let cmd = oneshot!(state_tx, CurrentCommand);

                if let (Some(items), Some(tabs), Some(cmd)) = (items, tabs, cmd) {
                    let mut events = vec![
                        (TypeId::of::<Tabs>(), Event::UpdateAll(tabs)),
                        (TypeId::of::<List>(), Event::UpdateAll(items.aliases())),
                    ];

                    if let Some(cmd) = cmd {
                        let event = (TypeId::of::<TextBox>(), Event::UpdateCommand(cmd.value));
                        let idx = (TypeId::of::<List>(), Event::UpdateListIdx(cmd.current_idx));

                        events.push(event);
                        events.push(idx);
                    };

                    Some(events)
                } else {
                    None
                }
            }
            ScreenCommandCallback::LoadCommandDetails(type_id) => {
                let cmd = oneshot!(state_tx,CommandDetails);
                if let Some(Some(cmd)) = cmd {
                    debug!("got cmd: {:?}", cmd);
                    Some(vec![(type_id, Event::UpdateCommand(cmd))])
                } else {
                    debug!("no command details found");
                    None
                }
            }
            _ => None,
        }
    }
}
