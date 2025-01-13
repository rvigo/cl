use cl_core::Command;
use crate::observer::event::Event;

#[derive(Debug, Clone)]
pub struct TextboxEvent {
    pub command: Command<'static>,
}

impl TextboxEvent {
    pub fn new(command: Command<'static>) -> Self {
        Self { command }
    }
}

impl Event for TextboxEvent {}
