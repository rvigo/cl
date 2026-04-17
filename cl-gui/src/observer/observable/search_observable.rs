use crate::component::Search;
use crate::observer::event::{Event, SearchEvent};
use crate::observer::observable::{Observable, ObservableFuture};
use crate::state::state_event::StateEvent::Filter;
use tracing::debug;

impl Observable for Search {
    fn on_listen(&mut self, event: Event) -> Option<ObservableFuture> {
        if let Event::Search(e) = event {
            match e {
                SearchEvent::Input(key, state_tx) => {
                    self.textarea.input(key);
                    let cur_input = self.textarea.lines().join("\n");
                    return Some(Box::pin(async move {
                        if let Err(e) = state_tx.send(Filter(cur_input)).await {
                            tracing::error!("Search: failed to send filter event: {e}");
                        }
                    }));
                }
                SearchEvent::UpdateQuery(query) => {
                    debug!("Search: pre-populating from query '{}'", query);
                    self.textarea.insert_str(query);
                }
            }
        }
        None
    }
}
