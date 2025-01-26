mod main_screen_key_mapping;
mod popup_key_mapping;

use crate::component::{List, Tabs, TextBox};
use crate::observer::event::Event;
use crate::oneshot;
use crate::screen::layer::Layer;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{CurrentCommand, GetAllListItems, GetAllNamespaces};
use crate::ui::ui_actor::CommandVecExt;
use async_trait::async_trait;
use crossterm::event::KeyEvent;
use std::any::TypeId;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

#[macro_export]
macro_rules! event {
    {$type:ty, $event:expr} => {
        ScreenCommand::Notify((std::any::TypeId::of::<$type>(), $event))
    };
}

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
                let tabs = oneshot!(state_tx, GetAllNamespaces);
                let items = oneshot!(state_tx, GetAllListItems);
                let cmd = oneshot!(state_tx, CurrentCommand);

                if let (Some(tabs), Some(items), Some(cmd)) = (tabs, items, cmd) {
                    Some(vec![
                        (TypeId::of::<Tabs>(), Event::UpdateAll(tabs)),
                        (TypeId::of::<List>(), Event::UpdateAll(items.aliases())),
                        (TypeId::of::<TextBox>(), Event::UpdateCommand(cmd.value)),
                    ])
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
