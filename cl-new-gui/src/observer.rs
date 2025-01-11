use crate::textbox::TextBox;
use cl_core::Command;
use std::fmt::Debug;
use std::path::Component;
use std::{cell::RefCell, rc::Rc};

pub trait Listener {
    async fn update(&mut self, event: Event);
}

pub trait Publisher<O>
where
    O: Listener,
{
    fn get_listeners(&self) -> &Vec<Rc<RefCell<O>>>;

    fn get_listeners_mut(&mut self) -> &mut Vec<Rc<RefCell<O>>>;

    fn register(&mut self, listener: Rc<RefCell<O>>);

    async fn notify(&mut self, event: Event);
}

pub struct ComponentPublisher {
    component: TextBox,
}

impl Publisher<TextBox> for ComponentPublisher {
    fn get_listeners(&self) -> &Vec<Rc<RefCell<TextBox>>> {
        todo!()
    }

    fn get_listeners_mut(&mut self) -> &mut Vec<Rc<RefCell<TextBox>>> {
        todo!()
    }

    fn register(&mut self, listener: Rc<RefCell<TextBox>>) {
        todo!()
    }

    async fn notify(&mut self, event: Event) {
        self.component.update(event).await;
    }
}

impl Listener for TextBox {
    async fn update(&mut self, event: Event) {
        self.command = Some(event.command)
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub command: Command<'static>,
    pub highlight: bool,
    pub query: String,
}

impl Event {
    pub fn new(command: Command<'static>, highlight: bool, query: String) -> Self {
        Self {
            command,
            highlight,
            query,
        }
    }
}
