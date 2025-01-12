use crate::component::{List, TextBox, TextBoxName};
use crate::SharedCell;
use cl_core::Command;
use std::fmt::{Debug, Display};

pub trait Listener {
    fn get_id(&self) -> &impl Display;
    async fn update(&mut self, event: ObserverEvent);
}

pub trait Publisher<O>
where
    O: Listener,
{
    fn get_listeners(&self) -> &Vec<SharedCell<O>>;

    fn get_listeners_mut(&mut self) -> &mut Vec<SharedCell<O>>;

    fn register(&mut self, listener: SharedCell<O>);

    async fn notify(&mut self, event: ObserverEvent);
}

#[derive(Default)]
pub struct ComponentPublisher {
    components: Vec<SharedCell<TextBox>>, // TODO change to component and handle trait error with dyn Component (maybe its 'cause the async method)
}

impl Publisher<TextBox> for ComponentPublisher {
    fn get_listeners(&self) -> &Vec<SharedCell<TextBox>> {
        &self.components
    }

    fn get_listeners_mut(&mut self) -> &mut Vec<SharedCell<TextBox>> {
        &mut self.components
    }

    fn register(&mut self, listener: SharedCell<TextBox>) {
        self.components.push(listener);
    }

    async fn notify(&mut self, event: ObserverEvent) {
        for component in self.get_listeners_mut() {
            component.borrow_mut().update(event.clone()).await
        }
    }
}

impl Listener for TextBox {
    fn get_id(&self) -> &impl Display {
       &self.name 
    }

    async fn update(&mut self, event: ObserverEvent) {
        let content = match self.name {
            TextBoxName::Command => event.command.command.to_string(),
            TextBoxName::Description => event.command.description(),
            TextBoxName::Tags => event.command.tags_as_string(),
            TextBoxName::Namespace => event.command.namespace.to_string(),
        };

        self.update_content(content);
    }
}

#[derive(Debug, Clone)]
pub struct ObserverEvent {
    pub command: Command<'static>,
}

impl ObserverEvent {
    pub fn new(command: Command<'static>) -> Self {
        Self { command }
    }
}
