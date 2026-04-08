use crate::state::selected_command::SelectedCommand;
use crate::state::state_event::StateEvent;
use crate::state::State;
use anyhow::Result;
use cl_core::Config;
use tracing::{debug, error};
use tokio::sync::mpsc::Receiver;

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
            self.handle_message(message)?;
        }

        Ok(())
    }

    fn handle_message(&mut self, message: StateEvent) -> Result<()> {
        match message {
            StateEvent::SelectNextCommand { respond_to } => {
                let selected_command = self.state.next_item();
                let _ = respond_to.send(selected_command);
            }
            StateEvent::SelectPreviousCommand { respond_to } => {
                let selected_command = self.state.previous_item();
                let _ = respond_to.send(selected_command);
            }
            StateEvent::ExecuteCommand => self.state.execute(),
            StateEvent::GetAllListItems { respond_to } => {
                let all_items = self.state.get_all_items();
                let _ = respond_to.send(all_items);
            }
            StateEvent::CurrentCommand { respond_to } => {
                let selected_command = self.state.get_selected_command();
                let _ = respond_to.send(selected_command);
            }
            StateEvent::PreviousTab { respond_to } => {
                let (selected_namespace, commands) = self.state.previous_tab();
                let selected_command = match commands.first() {
                    Some(cmd) => SelectedCommand::new(cmd.clone(), 0),
                    None => {
                        error!(
                            "PreviousTab returned empty commands for namespace '{}'",
                            selected_namespace.name
                        );
                        SelectedCommand::default()
                    }
                };
                let _ = respond_to.send((selected_namespace, selected_command, commands));
            }
            StateEvent::NextTab { respond_to } => {
                let (selected_namespace, commands) = self.state.next_tab();
                let selected_command = match commands.first() {
                    Some(cmd) => SelectedCommand::new(cmd.clone(), 0),
                    None => {
                        error!(
                            "NextTab returned empty commands for namespace '{}'",
                            selected_namespace.name
                        );
                        SelectedCommand::default()
                    }
                };
                let _ = respond_to.send((selected_namespace, selected_command, commands));
            }
            StateEvent::GetAllNamespaces { respond_to } => {
                let namespaces = self.state.get_all_namespaces();
                let _ = respond_to.send(namespaces);
            }
            StateEvent::DeleteCommand { respond_to } => {
                let res = match self.state.delete_command() {
                    Ok(_) => (true, None),
                    Err(e) => (false, Some(e.to_string())),
                };
                let _ = respond_to.send(res);
            }
            StateEvent::Filter(query) => {
                debug!("filtering with query: {}", query);
                self.state.filter(query)
            }
            StateEvent::GetCurrentQuery { respond_to } => {
                let _ = respond_to.send(self.state.get_current_query());
            }
            StateEvent::CommandDetails { respond_to } => {
                let command = self.state.get_selected_command();
                let _ = respond_to.send(command.map(|selected| selected.value));
            }
            StateEvent::EditField(type_, content) => {
                debug!(target: "edit_state_actor", "Editing field: {:?} with content: {}", type_, content);
                self.state.set_editable_command(type_, content);
            }
            StateEvent::EditCommand => {
                debug!("do editing command");
                self.state.edit_command()?;
            }
            StateEvent::InsertCommand => {
                debug!("inserting new command");
                self.state.insert_command()?;
            }
        }

        Ok(())
    }
}
