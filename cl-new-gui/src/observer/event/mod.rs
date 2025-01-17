mod list_event;
mod popup_event;
mod tabs_event;
mod textbox_event;

pub use list_event::ListAction;
pub use list_event::ListEvent;
pub use popup_event::PopupAction;
pub use popup_event::PopupEvent;
pub use tabs_event::TabsEvent;
pub use textbox_event::TextboxEvent;

use std::any::Any;

/// Event marker
pub trait Event: Any {
    fn as_any(&self) -> &dyn Any
    where
        Self: Sized,
    {
        self
    }
}
