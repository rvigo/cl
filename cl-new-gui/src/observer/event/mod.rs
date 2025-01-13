mod list_event;
mod textbox_event;

pub use list_event::ListAction;
pub use list_event::ListEvent;
use std::any::Any;
pub use textbox_event::TextboxEvent;

/// Event marker
pub trait Event: Any {
    fn as_any(&self) -> &dyn Any
    where
        Self: Sized,
    {
        self
    }
}
