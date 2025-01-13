use crate::observer::event::Event;

#[derive(Clone)]
pub enum ListAction {
    Next(usize),
    Previous(usize),
    UpdateAll(Vec<String>),
}

#[derive(Clone)]
pub struct ListEvent {
    pub action: ListAction,
}

impl ListEvent {
    pub fn new(action: ListAction) -> Self {
        Self { action }
    }
}

impl Event for ListEvent {}
