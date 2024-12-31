use crate::widget::{text_field::FieldType, DisplayWidget};
use cl_core::Command;
use std::{cell::RefCell, rc::Rc};

pub trait Observer {
    type ContentType;

    fn update(&mut self, event: Event<Self::ContentType>);
}

pub trait Sub<O>
where
    O: Observer,
{
    fn register(&mut self, observer: Rc<RefCell<O>>);

    fn notify(&mut self, event: Event<O::ContentType>);
}

pub trait Subject<O>
where
    O: Observer,
{
    fn get_observers(&self) -> &Vec<Rc<RefCell<O>>>;

    fn get_observers_mut(&mut self) -> &mut Vec<Rc<RefCell<O>>>;

    fn register(&mut self, observer: Rc<RefCell<O>>);

    fn notify(&mut self, event: Event<O::ContentType>);
}

#[derive(Debug, Clone)]
pub struct Event<T> {
    pub content: T,
}

impl<T> Event<T> {
    pub fn new(content: T) -> Self {
        Event { content }
    }
}

#[derive(Clone)]
pub struct CommandEvent<'event> {
    pub command: Command<'event>,
    pub highlight: bool,
    pub query: String,
}

impl<'event> CommandEvent<'event> {
    pub fn new(command: Command<'event>, query: String, highlight: bool) -> Self {
        CommandEvent {
            command,
            highlight,
            query,
        }
    }
}

impl<'d> Observer for DisplayWidget<'d> {
    type ContentType = CommandEvent<'d>;

    fn update(&mut self, event: Event<Self::ContentType>) {
        let command = event.content.command;
        let content = match self.r#type {
            FieldType::Command => command.command.to_string(),
            FieldType::Namespace => command.namespace.to_string(),
            FieldType::Tags => command.tags_as_string(),
            FieldType::Description => command.description(),
            _ => "".to_owned(),
        };

        self.update_content(content);
        self.highlight(event.content.query);
        self.should_highlight = event.content.highlight;
    }
}
