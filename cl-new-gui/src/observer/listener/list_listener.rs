use crate::component::List;
use crate::observer::event::{ListAction, ListEvent};
use crate::observer::listener::{Listener, ListenerId};

impl Listener for List {
    type EventType = ListEvent;

    fn get_id() -> ListenerId {
        ListenerId("List".to_string())
    }

    async fn on_event(&mut self, event: ListEvent) {
        match &event.action {
            ListAction::Next(idx) => self.next(*idx),
            ListAction::Previous(idx) => self.previous(*idx),
            ListAction::UpdateAll(items) => self.items = items.to_vec(),
        }
    }
}
