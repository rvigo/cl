use crate::state::state::{SelectedCommand, SelectedNamespace};
use cl_core::Command;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum StateEvent {
    /// Select the next command in the list
    SelectNextCommand {
        respond_to: oneshot::Sender<SelectedCommand>,
    },
    /// Select the previous command in the list
    SelectPreviousCommand {
        respond_to: oneshot::Sender<SelectedCommand>,
    },
    /// Execute the selected command
    ExecuteCommand,
    /// Get all list items based on the current namespace
    GetAllListItems {
        respond_to: oneshot::Sender<Vec<Command<'static>>>,
    },
    /// Get all namespaces
    GetAllNamespaces {
        respond_to: oneshot::Sender<Vec<String>>,
    },
    /// Get the current selected command
    CurrentCommand {
        respond_to: oneshot::Sender<SelectedCommand>,
    },
    /// Get the previous tab info
    PreviousTab {
        respond_to: oneshot::Sender<(SelectedNamespace, SelectedCommand, Vec<Command<'static>>)>,
    },
    /// Get the next tab info
    NextTab {
        respond_to: oneshot::Sender<(SelectedNamespace, SelectedCommand, Vec<Command<'static>>)>,
    },
    /// Delete the command
    DeleteCommand
}
