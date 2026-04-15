mod form_screen_key_mapping;
mod main_screen_key_mapping;
mod popup_key_mapping;
mod search_key_mapping;

pub mod command;

use crate::observer::event::NotifyTarget;
use crate::screen::key_mapping::command::ScreenCommand;
use std::any::TypeId;

/// Build a `ScreenCommand::Notify` for a component that implements [`NotifyTarget`].
///
/// ```ignore
/// create_notify_command::<List>(ListEvent::Next(idx))
/// create_notify_command::<Popup>(PopupEvent::Create(dialog))
/// ```
pub fn create_notify_command<C: NotifyTarget>(payload: C::Payload) -> ScreenCommand {
    ScreenCommand::Notify((TypeId::of::<C>(), C::wrap(payload)))
}
