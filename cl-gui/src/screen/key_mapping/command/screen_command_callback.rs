use crate::component::{List, Tabs, TextBox};
use crate::observer::event::{EditableTextboxEvent, Event, ListEvent, TabsEvent, TextBoxEvent};
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{
    CommandDetails, CurrentCommand, GetAllListItems, GetAllNamespaces,
};
use cl_core::{Command, CommandVecExt};
use tracing::debug;
use std::any::TypeId;
use tokio::sync::mpsc::Sender;

/// Enables the communication between the upper layer and the lower layer via callback commands
#[derive(Clone, Debug)]
pub enum ScreenCommandCallback {
    /// Refreshes all info
    UpdateAll,
    /// Load command details
    LoadCommandDetails(TypeId),
    /// Save changes to the current command
    SaveChanges(Option<Command<'static>>),
    /// Do nothing
    DoNothing,
    /// Exit the edit screen and return to the main screen
    ExitEditScreen,
}

impl ScreenCommandCallback {
    pub async fn handle(self, state_tx: &Sender<StateEvent>) -> Option<Vec<(TypeId, Event)>> {
        match self {
            ScreenCommandCallback::UpdateAll => {
                let items = oneshot!(state_tx, GetAllListItems).ok();
                let tabs = oneshot!(state_tx, GetAllNamespaces).ok();
                let cmd = oneshot!(state_tx, CurrentCommand).ok();

                if let (Some(items), Some(tabs), Some(cmd)) = (items, tabs, cmd) {
                    let mut events = vec![
                        (
                            TypeId::of::<Tabs>(),
                            Event::Tabs(TabsEvent::UpdateAll(tabs)),
                        ),
                        (
                            TypeId::of::<List>(),
                            Event::List(ListEvent::UpdateAll(items.aliases())),
                        ),
                    ];

                    if let Some(cmd) = cmd {
                        let event = (
                            TypeId::of::<TextBox>(),
                            Event::TextBox(TextBoxEvent::UpdateCommand(cmd.value)),
                        );
                        let idx = (
                            TypeId::of::<List>(),
                            Event::List(ListEvent::UpdateListIdx(cmd.current_idx)),
                        );

                        events.push(event);
                        events.push(idx);
                    };

                    Some(events)
                } else {
                    None
                }
            }
            ScreenCommandCallback::LoadCommandDetails(type_id) => {
                let cmd = oneshot!(state_tx, CommandDetails);
                if let Ok(Some(cmd)) = cmd {
                    debug!("got cmd: {:?}", cmd);
                    Some(vec![(
                        type_id,
                        Event::EditableTextbox(EditableTextboxEvent::UpdateCommand(cmd)),
                    )])
                } else {
                    debug!("no command details found");
                    None
                }
            }
            _ => None,
        }
    }
}
