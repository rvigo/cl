use crate::component::{List, Tabs, TextBox};
use crate::observer::event::{EditableTextboxEvent, Event, ListEvent, TabsEvent, TextBoxEvent};
use crate::screen::key_mapping::command::ScreenCommand;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{
    CommandDetails, CurrentCommand, GetAllListItems, GetAllNamespaces,
};
use cl_core::CommandVecExt;
use std::any::TypeId;
use tokio::sync::mpsc::Sender;
use tracing::debug;

/// Enables the communication between the upper layer and the lower layer via callback commands
#[derive(Clone, Debug)]
pub enum ScreenCommandCallback {
    /// Refreshes all info
    UpdateAll,
    /// Load command details
    LoadCommandDetails(TypeId),
    /// Save changes to the current command
    SaveChanges(Option<cl_core::Command<'static>>),
    /// Do nothing
    DoNothing,
    /// Exit the edit screen and return to the main screen
    ExitEditScreen,
}

impl ScreenCommandCallback {
    pub async fn handle(self, state_tx: &Sender<StateEvent>) -> Option<Vec<ScreenCommand>> {
        match self {
            ScreenCommandCallback::UpdateAll => {
                let items = oneshot!(state_tx, GetAllListItems).ok();
                let tabs = oneshot!(state_tx, GetAllNamespaces).ok();
                let cmd = oneshot!(state_tx, CurrentCommand).ok();

                if let (Some(items), Some(tabs), Some(cmd)) = (items, tabs, cmd) {
                    let selected_idx = cmd.as_ref().map_or(0, |c| c.current_idx);

                    let mut events: Vec<ScreenCommand> = vec![
                        // Update the navigation snapshot first
                        ScreenCommand::SetSnapshot {
                            items: items.clone(),
                            selected_idx,
                        },
                        ScreenCommand::Notify((
                            TypeId::of::<Tabs>(),
                            Event::Tabs(TabsEvent::UpdateAll(tabs)),
                        )),
                        ScreenCommand::Notify((
                            TypeId::of::<List>(),
                            Event::List(ListEvent::UpdateAll(items.aliases())),
                        )),
                    ];

                    if let Some(cmd) = cmd {
                        events.push(ScreenCommand::Notify((
                            TypeId::of::<TextBox>(),
                            Event::TextBox(TextBoxEvent::UpdateCommand(cmd.value)),
                        )));
                        events.push(ScreenCommand::Notify((
                            TypeId::of::<List>(),
                            Event::List(ListEvent::UpdateListIdx(cmd.current_idx)),
                        )));
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
                    Some(vec![ScreenCommand::Notify((
                        type_id,
                        Event::EditableTextbox(EditableTextboxEvent::UpdateCommand(cmd)),
                    ))])
                } else {
                    debug!("no command details found");
                    None
                }
            }
            _ => None,
        }
    }
}
