use crate::component::Tabs;
use crate::observer::event::Event;
use crate::observer::observable::Observable;
use async_trait::async_trait;

#[async_trait(?Send)]
impl Observable for Tabs {
    async fn on_listen(&mut self, event: Event) {
        match event {
            Event::Next(idx) => self.next(idx),
            Event::Previous(idx) => self.previous(idx),
            Event::UpdateAll(items) => {
                self.update_items(items);
                self.reset_selected()
            }
            _ => {}
        }
    }
}
