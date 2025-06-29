use crate::component::List;
use crate::observer::event::Event;
use crate::observer::observable::Observable;
use async_trait::async_trait;

#[async_trait(?Send)]
impl Observable for List {
    async fn on_listen(&mut self, event: Event) {
        match &event {
            Event::Next(idx) => self.next(*idx),
            Event::Previous(idx) => self.previous(*idx),
            Event::UpdateAll(items) => {
                self.update_items(items.to_vec());
                self.state.select(Some(0))
            }
            Event::UpdateListIdx(idx) =>{
                self.state.select(Some(*idx))
            }
            _ => {}
        }
    }
}
