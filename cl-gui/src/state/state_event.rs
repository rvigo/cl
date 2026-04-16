use crate::state::selected_command::SelectedCommand;
use crate::state::selected_namespace::SelectedNamespace;
use cl_core::Command;
use std::fmt;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum StateEvent {
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
        respond_to: oneshot::Sender<Option<SelectedCommand>>,
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
    DeleteCommand {
        respond_to: oneshot::Sender<Result<(), String>>,
    },
    /// Filter
    Filter(String),
    /// Get current query
    GetCurrentQuery { respond_to: oneshot::Sender<String> },
    /// Load command details in the current layer
    CommandDetails {
        respond_to: oneshot::Sender<Option<Command<'static>>>,
    },
    /// Edit Field
    EditField(FieldName, String),
    /// Edit Command
    EditCommand {
        respond_to: oneshot::Sender<Result<(), String>>,
    },
    /// Insert a new command
    InsertCommand {
        respond_to: oneshot::Sender<Result<(), String>>,
    },
    /// Sync the selected index from UI-local navigation (fire-and-forget)
    SyncSelection(usize),
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub enum FieldName {
    /// Edit the command description
    Description,
    /// Edit the command alias
    #[default]
    Alias,
    /// Edit the command tags
    Tags,
    /// Edit the command
    Command,
    /// Edit the command namespace
    Namespace,
}

impl fmt::Display for FieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldName::Description => write!(f, "Description"),
            FieldName::Alias => write!(f, "Alias"),
            FieldName::Tags => write!(f, "Tags"),
            FieldName::Command => write!(f, "Command"),
            FieldName::Namespace => write!(f, "Namespace"),
        }
    }
}
