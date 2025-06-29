use std::any::TypeId;
use tokio::sync::mpsc::Receiver;
use crate::observer::event::Event;
use crate::screen::key_mapping::command::{EditCallback, ScreenCommandCallback};
use crate::screen::layer::Layer;

/// Commands that can be sent to the current layer
pub enum ScreenCommand {
    /// Notify the current layer listeners with an event
    Notify((TypeId, Event)),
    /// Add a new layer to the screen
    AddLayer(Box<dyn Layer + 'static>),
    /// Pop the last layer from the screen and send a callback to the previous layer
    PopLastLayer(Option<Receiver<ScreenCommandCallback>>),
    /// Replace the current one
    ReplaceCurrentLayer(Box<dyn Layer + 'static>),
    /// Copy content to clipboard
    CopyToClipboard,
    /// Callback,
    Callback(ScreenCommandCallback),
    /// Get content from the editable textbox
    GetFieldContent,
    /// Edit the current command
    Edit(EditCallback),
    /// Quit the app
    Quit,
}

impl From<(TypeId, Event)> for ScreenCommand {
    fn from(value: (TypeId, Event)) -> Self {
        ScreenCommand::Notify(value)
    }
}
