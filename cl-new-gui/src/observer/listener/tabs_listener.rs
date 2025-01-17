use crate::component::Tabs;
use crate::observer::event::TabsEvent;
use crate::observer::listener::Observable;

impl Observable for Tabs {
    type EventType = TabsEvent;

    fn on_listen(&mut self, event: Self::EventType) {
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
