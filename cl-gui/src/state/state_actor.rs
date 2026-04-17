use crate::state::selected_command::SelectedCommand;
use crate::state::state_event::StateEvent;
use crate::state::State;
use anyhow::Result;
use cl_core::Config;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error};

/// Send a response on a oneshot channel, logging a debug message if the
/// receiver has already been dropped.
macro_rules! respond {
    ($tx:expr, $val:expr, $label:literal) => {
        if $tx.send($val).is_err() {
            debug!(concat!($label, ": response receiver dropped"));
        }
    };
}

pub struct StateActor {
    state: State,
    receiver: Receiver<StateEvent>,
}

impl StateActor {
    pub fn new(config: impl Config + 'static, receiver: Receiver<StateEvent>) -> Result<Self> {
        Ok(Self {
            state: State::new(config)?,
            receiver,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        while let Some(message) = self.receiver.recv().await {
            self.handle_message(message).await?;
        }

        Ok(())
    }

    async fn handle_message(&mut self, message: StateEvent) -> Result<()> {
        match message {
            StateEvent::ExecuteCommand => self.state.execute(),
            StateEvent::GetAllListItems { respond_to } => {
                let all_items = self.state.get_all_items().clone();
                respond!(respond_to, all_items, "GetAllListItems");
            }
            StateEvent::CurrentCommand { respond_to } => {
                let selected_command = self.state.get_selected_command().cloned();
                respond!(respond_to, selected_command, "CurrentCommand");
            }
            StateEvent::PreviousTab { respond_to } => {
                let (selected_namespace, commands) = self.state.previous_tab();
                let selected_command =
                    self.state
                        .get_selected_command()
                        .cloned()
                        .unwrap_or_else(|| {
                            error!(
                                "PreviousTab: no selected command for namespace '{}'",
                                selected_namespace.name
                            );
                            SelectedCommand::default()
                        });
                respond!(
                    respond_to,
                    (selected_namespace, selected_command, commands),
                    "PreviousTab"
                );
            }
            StateEvent::NextTab { respond_to } => {
                let (selected_namespace, commands) = self.state.next_tab();
                let selected_command =
                    self.state
                        .get_selected_command()
                        .cloned()
                        .unwrap_or_else(|| {
                            error!(
                                "NextTab: no selected command for namespace '{}'",
                                selected_namespace.name
                            );
                            SelectedCommand::default()
                        });
                respond!(
                    respond_to,
                    (selected_namespace, selected_command, commands),
                    "NextTab"
                );
            }
            StateEvent::GetAllNamespaces { respond_to } => {
                let namespaces = self.state.get_all_namespaces().to_vec();
                respond!(respond_to, namespaces, "GetAllNamespaces");
            }
            StateEvent::DeleteCommand { respond_to } => {
                let res = self.state.delete_command().await.map_err(|e| e.to_string());
                respond!(respond_to, res, "DeleteCommand");
            }
            StateEvent::Filter(query) => {
                debug!("filtering with query: {}", query);
                self.state.filter(&query)
            }
            StateEvent::GetCurrentQuery { respond_to } => {
                respond!(
                    respond_to,
                    self.state.get_current_query(),
                    "GetCurrentQuery"
                );
            }
            StateEvent::CommandDetails { respond_to } => {
                let command = self.state.get_selected_command().map(|s| s.value.clone());
                respond!(respond_to, command, "CommandDetails");
            }
            StateEvent::EditField(type_, content) => {
                debug!(target: "edit_state_actor", "Editing field: {:?} with content: {}", type_, content);
                self.state.set_editable_command(type_, content);
            }
            StateEvent::EditCommand { respond_to } => {
                debug!("editing command");
                let result = self.state.edit_command().await.map_err(|e| e.to_string());
                if let Err(ref e) = result {
                    error!("Failed to edit command: {}", e);
                }
                respond!(respond_to, result, "EditCommand");
            }
            StateEvent::InsertCommand { respond_to } => {
                debug!("inserting new command");
                let result = self.state.insert_command().await.map_err(|e| e.to_string());
                if let Err(ref e) = result {
                    error!("Failed to insert command: {}", e);
                }
                respond!(respond_to, result, "InsertCommand");
            }
            StateEvent::SyncSelection(idx) => {
                debug!("syncing selection to index {}", idx);
                self.state.select(idx);
            }
        }

        Ok(())
    }
}
