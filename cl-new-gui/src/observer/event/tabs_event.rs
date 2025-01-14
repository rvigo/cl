use crate::observer::event::Event;

#[derive(Clone)]
pub enum TabsEvent {
    UpdateItems(Vec<String>),
    Next(usize),
    Previous(usize),
}

impl Event for TabsEvent {}
