use crate::component::List;
use crate::observer::publisher::Publisher;
use crate::SharedCell;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListPublisher {
    listeners: Vec<SharedCell<List>>,
}

impl ListPublisher {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
}

impl Publisher<List> for ListPublisher {
    fn get_listeners(&self) -> &Vec<SharedCell<List>> {
        &self.listeners
    }

    fn get_listeners_mut(&mut self) -> &mut Vec<SharedCell<List>> {
        &mut self.listeners
    }

    fn register(&mut self, listener: SharedCell<List>) {
        self.listeners.push(listener);
    }
}
