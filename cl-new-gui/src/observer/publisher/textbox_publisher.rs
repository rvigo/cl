use crate::component::TextBox;
use crate::observer::publisher::Publisher;
use crate::SharedCell;

#[derive(Clone, Debug, Default, Eq, PartialEq, )]
pub struct TextBoxPublisher {
    listeners: Vec<SharedCell<TextBox>>,
}

impl Publisher<TextBox> for TextBoxPublisher {
    fn get_listeners(&self) -> &Vec<SharedCell<TextBox>> {
        &self.listeners
    }

    fn get_listeners_mut(&mut self) -> &mut Vec<SharedCell<TextBox>> {
        &mut self.listeners
    }

    fn register(&mut self, listener: SharedCell<TextBox>) {
        self.listeners.push(listener);
    }
}
