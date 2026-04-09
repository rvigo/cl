use crate::component::FutureEventType;
use crate::screen::command::ScreenCommandCallback;
use crate::screen::ActiveScreen;
use crate::state::state_event::{FieldName, StateEvent};
use cl_core::Command;
use tokio::sync::mpsc::Sender;

/// Top-level event enum. Each variant targets a specific component type.
/// The TypeId in ScreenCommand::Notify routes to the right subscribers;
/// the variant here determines what the subscriber does with it.
#[derive(Clone, Debug)]
pub enum Event {
    List(ListEvent),
    Tabs(TabsEvent),
    TextBox(TextBoxEvent),
    EditableTextbox(EditableTextboxEvent),
    ScreenState(ScreenStateEvent),
    Popup(PopupEvent),
    Search(SearchEvent),
    ClipboardStatus(ClipboardAction),
}

// ---------------------------------------------------------------------------
// Per-component event enums
// ---------------------------------------------------------------------------

/// Events handled by the [`List`](crate::component::List) component.
#[derive(Clone, Debug)]
pub enum ListEvent {
    Next(usize),
    Previous(usize),
    UpdateAll(Vec<String>),
    UpdateListIdx(usize),
}

/// Events handled by the [`Tabs`](crate::component::Tabs) component.
#[derive(Clone, Debug)]
pub enum TabsEvent {
    Next(usize),
    Previous(usize),
    UpdateAll(Vec<String>),
}

/// Events handled by read-only [`TextBox`](crate::component::TextBox) components.
#[derive(Clone, Debug)]
pub enum TextBoxEvent {
    UpdateCommand(Command<'static>),
    UpdateContent(String),
}

/// Events handled by [`EditableTextbox`](crate::component::EditableTextbox) components.
#[derive(Clone, Debug)]
pub enum EditableTextboxEvent {
    UpdateCommand(Command<'static>),
    KeyInput(crossterm::event::KeyEvent),
    GetFieldContent(Sender<StateEvent>),
    SetField(FieldName),
}

/// Events handled by the [`ScreenState`](crate::component::ScreenState) component.
#[derive(Clone, Debug)]
pub enum ScreenStateEvent {
    SetField(FieldName),
    KeyInput(crossterm::event::KeyEvent),
}

/// Events handled by the [`Search`](crate::component::Search) component.
#[derive(Clone, Debug)]
pub enum SearchEvent {
    Input(crossterm::event::KeyEvent, Sender<StateEvent>),
    UpdateQuery(String),
}

// ---------------------------------------------------------------------------
// Popup events (kept from original, already well-typed)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum PopupEvent {
    Create(PopupType),
    NextChoice,
    PreviousChoice,
    Run(Sender<StateEvent>, Sender<ScreenCommandCallback>),
}

#[derive(Clone, Debug)]
pub enum PopupType {
    Dialog(String, FutureEventType, ScreenCommandCallback),
    Help(ActiveScreen),
}

// ---------------------------------------------------------------------------
// Clipboard
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum ClipboardAction {
    Copied,
}
