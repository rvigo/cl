use crate::state::selected_command::SelectedCommand;
use crate::state::state_event::StateEvent;
use crate::state::State;
use anyhow::Result;
use cl_core::Config;
use tokio::sync::mpsc::Receiver;
use tracing::{debug, error};

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
                if respond_to.send(all_items).is_err() {
                    debug!("GetAllListItems: response receiver dropped");
                }
            }
            StateEvent::CurrentCommand { respond_to } => {
                let selected_command = self.state.get_selected_command().cloned();
                if respond_to.send(selected_command).is_err() {
                    debug!("CurrentCommand: response receiver dropped");
                }
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
                if respond_to
                    .send((selected_namespace, selected_command, commands))
                    .is_err()
                {
                    debug!("PreviousTab: response receiver dropped");
                }
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
                if respond_to
                    .send((selected_namespace, selected_command, commands))
                    .is_err()
                {
                    debug!("NextTab: response receiver dropped");
                }
            }
            StateEvent::GetAllNamespaces { respond_to } => {
                let namespaces = self.state.get_all_namespaces().to_vec();
                if respond_to.send(namespaces).is_err() {
                    debug!("GetAllNamespaces: response receiver dropped");
                }
            }
            StateEvent::DeleteCommand { respond_to } => {
                let res = self.state.delete_command().await.map_err(|e| e.to_string());
                if respond_to.send(res).is_err() {
                    debug!("DeleteCommand: response receiver dropped");
                }
            }
            StateEvent::Filter(query) => {
                debug!("filtering with query: {}", query);
                self.state.filter(&query)
            }
            StateEvent::GetCurrentQuery { respond_to } => {
                if respond_to.send(self.state.get_current_query()).is_err() {
                    debug!("GetCurrentQuery: response receiver dropped");
                }
            }
            StateEvent::CommandDetails { respond_to } => {
                let command = self.state.get_selected_command().map(|s| s.value.clone());
                if respond_to.send(command).is_err() {
                    debug!("CommandDetails: response receiver dropped");
                }
            }
            StateEvent::EditField(type_, content) => {
                debug!(target: "edit_state_actor", "Editing field: {:?} with content: {}", type_, content);
                self.state.set_editable_command(type_, content);
            }
            StateEvent::EditCommand { respond_to } => {
                debug!("editing command");
                let result = self.state.edit_command().await.map_err(|e| e.to_string());
                if respond_to.send(result.clone()).is_err() {
                    debug!("EditCommand: response receiver dropped");
                }
                if let Err(ref e) = result {
                    error!("Failed to edit command: {}", e);
                }
            }
            StateEvent::InsertCommand { respond_to } => {
                debug!("inserting new command");
                let result = self.state.insert_command().await.map_err(|e| e.to_string());
                if respond_to.send(result.clone()).is_err() {
                    debug!("InsertCommand: response receiver dropped");
                }
                if let Err(ref e) = result {
                    error!("Failed to insert command: {}", e);
                }
            }
            StateEvent::SyncSelection(idx) => {
                debug!("syncing selection to index {}", idx);
                self.state.select(idx);
            }
        }

        Ok(())
    }
}
