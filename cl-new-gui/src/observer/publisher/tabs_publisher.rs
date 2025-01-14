use crate::component::Tabs;
use crate::observer::publisher::Publisher;
use crate::SharedCell;

#[derive(Debug, Eq, PartialEq)]
pub struct TabsPublisher {
    listeners: Vec<SharedCell<Tabs>>,
}

impl TabsPublisher {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }
}

impl Publisher<Tabs> for TabsPublisher {
    fn get_listeners(&self) -> &Vec<SharedCell<Tabs>> {
        &self.listeners
    }

    fn get_listeners_mut(&mut self) -> &mut Vec<SharedCell<Tabs>> {
        &mut self.listeners
    }

    fn register(&mut self, listener: SharedCell<Tabs>) {
        self.listeners.push(listener)
    }
}
