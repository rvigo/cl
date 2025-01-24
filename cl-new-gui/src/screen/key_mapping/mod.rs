mod main_screen;
mod popup;

use crate::observer::event::Event;
use crate::screen::layer::Layer;
use crate::state::state_event::StateEvent;
use async_trait::async_trait;
use crossterm::event::KeyEvent;
use std::any::TypeId;
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

pub enum ScreenCommand {
    Notify((TypeId, Event)),
    AddLayer(Box<dyn Layer + 'static>),
    PopLastLayer,
    Quit,
}

impl From<(TypeId, Event)> for ScreenCommand {
    fn from(value: (TypeId, Event)) -> Self {
        ScreenCommand::Notify(value)
    }
}

pub trait ScreenCommandVecExt {
    fn into_notify(self) -> Vec<ScreenCommand>;
}

impl ScreenCommandVecExt for Vec<(TypeId, Event)> {
    fn into_notify(self) -> Vec<ScreenCommand> {
        self.into_iter().map(ScreenCommand::from).collect()
    }
}
