mod list_listener;
mod textbox_listener;

use crate::observer::event::Event;

#[derive(Eq, Hash, PartialEq)]
pub struct ListenerId(pub String);

pub trait Listener {
    type EventType: Event + Sized;

    fn get_id() -> ListenerId;

    async fn on_event(&mut self, event: Self::EventType);
}
