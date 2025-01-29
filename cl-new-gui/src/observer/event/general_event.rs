use crate::screen::{ActiveScreen, ScreenCommandCallback};
use crate::state::state_event::StateEvent;
use cl_core::Command;
use crossterm::event::KeyEvent;
use tokio::sync::mpsc::Sender;

// TODO rethink the name of these events
#[derive(Clone, Debug)]
pub enum Event {
    Next(usize),
    Previous(usize),
    UpdateAll(Vec<String>),
    UpdateCommand(Command<'static>),
    UpdateContent(String),
    Popup(PopupEvent),
    Clipboard(ClipboardAction),
    Search(SearchAction, Sender<StateEvent>),
    UpdateQuery(String),
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

#[derive(Clone, Debug)]
pub enum SearchAction {
    Input(KeyEvent),
}
