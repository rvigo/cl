use crate::component::Tabs;
use crate::observer::event::TabsEvent;
use crate::observer::listener::{Listener, ListenerId};

impl Listener for Tabs {
    type EventType = TabsEvent;

    fn get_id() -> ListenerId {
        ListenerId("Tabs".to_string())
    }

    async fn on_event(&mut self, event: Self::EventType) {
        match event {
            TabsEvent::Next(idx) => self.next(idx),
            TabsEvent::Previous(idx) => self.previous(idx),
            TabsEvent::UpdateItems(items) => {
                self.items = items;
                self.selected = 0
            }
        }
    }
}
