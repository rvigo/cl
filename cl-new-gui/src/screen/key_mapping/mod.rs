use cl_core::CommandVecExt;
mod main_screen_key_mapping;
mod popup_key_mapping;
mod search_key_mapping;

use crate::component::{List, Tabs, TextBox};
use crate::observer::event::Event;
use crate::oneshot;
use crate::screen::layer::Layer;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{
    CurrentCommand, GetAllListItems, GetAllNamespaces,
};
use async_trait::async_trait;
use crossterm::event::KeyEvent;
use std::any::TypeId;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

#[async_trait(?Send)]
pub trait KeyMapping {
    async fn handle_key_event(
        &self,
        key: KeyEvent,
        state_tx: Sender<StateEvent>,
    ) -> Option<Vec<ScreenCommand>>;
}

/// Commands that can be sent to the current layer
pub enum ScreenCommand {
    /// Notify the current layer listeners with an event
    Notify((TypeId, Event)),
    /// Add a new layer to the screen
    AddLayer(Box<dyn Layer + 'static>),
    /// Pop the last layer from the screen and send a callback to the previous layer
    PopLastLayer(Option<Receiver<ScreenCommandCallback>>),
    /// Copy content to clipboard
    CopyToClipboard,
    /// Callback,
    Callback(ScreenCommandCallback),
    /// Quit the app
    Quit,
}

/// Enables the communication between the upper layer and the lower layer via callback commands
pub enum ScreenCommandCallback {
    /// Refreshes all info
    UpdateAll,
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
                        events.push(event)
                    };

                    Some(events)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl From<(TypeId, Event)> for ScreenCommand {
    fn from(value: (TypeId, Event)) -> Self {
        ScreenCommand::Notify(value)
    }
}
