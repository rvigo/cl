mod form_screen_key_mapping;
mod main_screen_key_mapping;
mod popup_key_mapping;
mod search_key_mapping;

pub mod command;

use crate::observer::event::NotifyTarget;
use crate::screen::key_mapping::command::ScreenCommand;
use crate::state::state_event::StateEvent;
use crossterm::event::KeyEvent;
use std::any::TypeId;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::Sender;

/// Build a `ScreenCommand::Notify` for a component that implements [`NotifyTarget`].
///
/// ```ignore
/// create_notify_command::<List>(ListEvent::Next(idx))
/// create_notify_command::<Popup>(PopupEvent::Create(dialog))
/// ```
pub fn create_notify_command<C: NotifyTarget>(payload: C::Payload) -> ScreenCommand {
    ScreenCommand::Notify((TypeId::of::<C>(), C::wrap(payload)))
}

pub trait KeyMapping {
    fn handle_key_event<'a>(
        &'a self,
        key: KeyEvent,
        state_tx: Sender<StateEvent>,
    ) -> Pin<Box<dyn Future<Output = Option<Vec<ScreenCommand>>> + 'a>>;
}
