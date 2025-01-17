pub mod publisher_container;

use crate::observer::listener::{Listener, Observable};

pub struct Publisher<O> {
    subscriber_set: Vec<Listener<O>>,
}

impl<O> Publisher<O>
where
    O: Observable,
{
    pub fn new() -> Self {
        Self {
            subscriber_set: Vec::new(),
        }
    }

    pub fn register(&mut self, listener: Listener<O>) {
        self.subscriber_set.push(listener);
    }

    async fn notify(&mut self, event: O::EventType) {
        self.subscriber_set.iter_mut().for_each(|listener| {
            listener.listen(event.clone());
        });
    }
}
