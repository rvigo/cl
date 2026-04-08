use crate::component::Search;
use crate::observer::event::{Event, SearchEvent};
use crate::observer::observable::Observable;
use crate::state::state_event::StateEvent::Filter;
use async_trait::async_trait;
use tracing::debug;

#[async_trait(?Send)]
impl Observable for Search {
    async fn on_listen(&mut self, event: Event) {
        if let Event::Search(e) = event {
            match e {
                SearchEvent::Input(key, state_tx) => {
                    self.textarea.input(key);
                    let cur_input = self.textarea.lines().join("\n");
                    state_tx.send(Filter(cur_input)).await.ok();
                }
                SearchEvent::UpdateQuery(query) => {
                    debug!("Search: pre-populating from query '{}'", query);
                    self.textarea.insert_str(query);
                }
            }
        }
    }
}
