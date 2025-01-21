use crate::component::List;
use crate::observer::event::Event;
use crate::observer::observable::Observable;
use crate::observer::ObservableComponent;

impl Observable for List {
    fn on_listen(&mut self, event: Event) {
        match &event {
            Event::Next(idx) => self.next(*idx),
            Event::Previous(idx) => self.previous(*idx),
            Event::UpdateAll(items) => {
                self.items = items.to_vec();
                self.state.select(Some(0))
            }
            _ => {}
        }
    }
}

impl ObservableComponent for List {}