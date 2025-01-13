mod list_publisher;
mod textbox_publisher;

use crate::component::{List, TextBox};
use crate::observer::event::Event;
use crate::observer::listener::Listener;
use crate::SharedCell;
pub use list_publisher::ListPublisher;
pub use textbox_publisher::TextBoxPublisher;

pub trait Publisher<O>
where
    O: Listener,
    O::EventType: Clone,
{
    fn get_listeners(&self) -> &Vec<SharedCell<O>>;

    fn get_listeners_mut(&mut self) -> &mut Vec<SharedCell<O>>;

    fn register(&mut self, listener: SharedCell<O>);

    async fn notify(&mut self, event: O::EventType) {
        for listener in self.get_listeners_mut() {
            listener.borrow_mut().on_event(event.clone()).await
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum PublisherContainer {
    TextBox(TextBoxPublisher),
    List(ListPublisher),
}

impl PublisherContainer {
    pub async fn notify(&mut self, event: impl Event) {
        match self {
            PublisherContainer::TextBox(p) => {
                if let Some(inner) = event
                    .as_any()
                    .downcast_ref::<<TextBox as Listener>::EventType>()
                {
                    p.notify(inner.clone()).await;
                }
            }

            PublisherContainer::List(p) => {
                if let Some(inner) = event
                    .as_any()
                    .downcast_ref::<<List as Listener>::EventType>()
                {
                    p.notify(inner.clone()).await;
                }
            }
        }
    }
}
