use crate::screen::{ActiveScreen, ScreenCommandCallback};
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
    Clipboard(ClipboardAction),
}

#[derive(Clone, Debug)]
pub enum PopupEvent {
    Create(PopupType),
    NextChoice,
    PreviousChoice,
    Run(Sender<StateEvent>, Sender<ScreenCommandCallback>),
}

#[derive(Clone, Debug)]
pub enum PopupType {
    Dialog(String),
    Help(ActiveScreen),
}

#[derive(Clone, Debug)]
pub enum ClipboardAction {
    Copied,
}
