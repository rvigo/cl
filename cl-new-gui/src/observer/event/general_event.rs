use crate::state::state_event::StateEvent;
use cl_core::Command;
use tokio::sync::mpsc::Sender;

// TODO rethink the name of these events

#[derive(Clone, Debug)]
pub enum Event {
    Next(usize),
    Previous(usize),
    UpdateAll(Vec<String>),
    UpdateCommand(Command<'static>),
    Popup(PopupEvent),
}

#[derive(Clone, Debug)]
pub enum PopupEvent {
    Create(PopupType),
    NextChoice,
    PreviousChoice,
    Run(Sender<StateEvent>),
    Action(PopupAction),
}

#[derive(Clone, Debug)]
pub enum PopupAction {
    Confirm,
    Cancel,
}

#[derive(Clone, Debug)]
pub enum PopupType {
    Dialog(String),
}
