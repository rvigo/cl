use crate::observer::event::Event;
use cl_core::Command;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TextboxEvent {
    UpdateCommand(Command<'static>),
}

impl Event for TextboxEvent {}
