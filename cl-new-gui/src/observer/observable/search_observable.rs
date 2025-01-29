use crate::component::Search;
use crate::observer::event::{Event, SearchAction};
use crate::observer::observable::Observable;
use crate::state::state_event::StateEvent::Filter;
use async_trait::async_trait;
use log::debug;

#[async_trait(?Send)]
impl Observable for Search {
    async fn on_listen(&mut self, event: Event) {
        match event {
            Event::Search(action, state_tx) => match action {
                SearchAction::Input(event_key) => {
                    self.textarea.input(event_key);

                    let cur_input = self.textarea.lines().join("\n");
                    state_tx.send(Filter(cur_input)).await.ok();
                }
            },
            Event::UpdateQuery(query) => {
                debug!("updating query from state: {}", query);
                self.textarea.insert_str(query);
            }
            _ => {}
        }
    }
}
