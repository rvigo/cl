use crate::state::selected_command::SelectedCommand;
use crate::state::selected_namespace::SelectedNamespace;
use cl_core::Command;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum StateEvent {
    /// Select the next command in the list
    SelectNextCommand {
        respond_to: oneshot::Sender<Option<SelectedCommand>>,
    },
    /// Select the previous command in the list
    SelectPreviousCommand {
        respond_to: oneshot::Sender<Option<SelectedCommand>>,
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
        respond_to: oneshot::Sender<(bool, Option<String>)>,
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
    EditField(FieldType, String),
    /// Edit Command
    EditCommand,
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub enum FieldType {
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
