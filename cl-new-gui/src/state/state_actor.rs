use crate::state::selected_command::SelectedCommand;
use crate::state::state::State;
use crate::state::state_event::StateEvent;
use anyhow::Result;
use cl_core::Config;
use log::debug;
use tokio::sync::mpsc::Receiver;

pub struct StateActor {
    state: State,
    receiver: Receiver<StateEvent>,
}

impl StateActor {
    pub fn new(
        config: impl Config + 'static,
        receiver: Receiver<StateEvent>
    ) -> Self {
        Self {
            state: State::new(config),
            receiver,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        while let Some(message) = self.receiver.recv().await {
            self.handle_message(message);
        }

        Ok(())
    }

    fn handle_message(&mut self, message: StateEvent) {
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
                let selected_namespace = self.state.previous_tab();
                let commands = self.state.current_commands();
                let selected_command = SelectedCommand::new(commands[0].clone(), 0);
                let _ = respond_to.send((selected_namespace, selected_command, commands));
            }
            StateEvent::NextTab { respond_to } => {
                let selected_namespace = self.state.next_tab();
                let commands = self.state.current_commands();
                let selected_command = SelectedCommand::new(commands[0].clone(), 0);
                let _ = respond_to.send((selected_namespace, selected_command, commands));
            }
            StateEvent::GetAllNamespaces { respond_to } => {
                let namespaces = self.state.get_all_namespaces();
                let _ = respond_to.send(namespaces);
            }
            StateEvent::DeleteCommand => self.state.delete_command(),
            StateEvent::Filter(query) => {
                debug!("filtering with query: {}", query);
                self.state.filter(query)
            }
            StateEvent::GetCurrentQuery { respond_to } => {
                let _ = respond_to.send(self.state.get_current_query());
            }
        }
    }
}
