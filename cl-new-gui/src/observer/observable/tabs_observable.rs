use crate::component::Tabs;
use crate::observer::event::Event;
use crate::observer::observable::Observable;
use crate::observer::ObservableComponent;

impl Observable for Tabs {
    fn on_listen(&mut self, event: Event) {
        match event {
            Event::Next(idx) => self.next(idx),
            Event::Previous(idx) => self.previous(idx),
            Event::UpdateAll(items) => {
                self.items = items;
                self.selected = 0
            }
            _ => {}
        }
    }
}

impl ObservableComponent for Tabs {}
