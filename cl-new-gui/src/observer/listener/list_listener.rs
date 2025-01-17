use crate::component::List;
use crate::observer::event::{ListAction, ListEvent};
use crate::observer::listener::Observable;

impl Observable for List {
    type EventType = ListEvent;

    fn on_listen(&mut self, event: Self::EventType) {
        match &event.action {
            ListAction::Next(idx) => self.next(*idx),
            ListAction::Previous(idx) => self.previous(*idx),
            ListAction::UpdateAll(items) => {
                self.items = items.to_vec();
                self.state.select(Some(0))
            }
        }
    }
}
