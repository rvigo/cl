mod list_event;
mod textbox_event;
mod tabs_event;

pub use list_event::ListAction;
pub use list_event::ListEvent;
pub use textbox_event::TextboxEvent;
pub use tabs_event::TabsEvent;

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
