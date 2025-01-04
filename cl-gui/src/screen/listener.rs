use crate::widget::{text_field::FieldType, DisplayWidget};
use cl_core::Command;
use std::{cell::RefCell, rc::Rc};

pub trait Listener {
    fn update(&mut self, event: Event);
}

pub trait Publisher<O>
where
    O: Listener,
{
    fn get_listeners(&self) -> &Vec<Rc<RefCell<O>>>;

    fn get_listeners_mut(&mut self) -> &mut Vec<Rc<RefCell<O>>>;

    fn register(&mut self, listener: Rc<RefCell<O>>);

    fn notify(&mut self, event: Event);
}

impl<'d> Listener for DisplayWidget<'d> {
    fn update(&mut self, event: Event) {
        let command = event.command;
        let content = match self.r#type {
            FieldType::Command => command.command.to_string(),
            FieldType::Namespace => command.namespace.to_string(),
            FieldType::Tags => command.tags_as_string(),
            FieldType::Description => command.description(),
            _ => unreachable!(),
        };

        self.update_content(content);
        self.highlight(event.query);
        self.should_highlight = event.highlight;
    }
}

#[derive(Debug, Clone)]
pub struct Event<'event> {
    pub command: Command<'event>,
    pub highlight: bool,
    pub query: String,
}

impl<'event> Event<'event> {
    pub fn new(command: Command<'event>, highlight: bool, query: String) -> Self {
        Self {
            command,
            highlight,
            query,
        }
    }
}
