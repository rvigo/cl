use crate::state::state::{SelectedCommand, State};
use crate::state::state_event::StateEvent;
use anyhow::Result;
use tokio::sync::mpsc::Receiver;

pub struct StateActor {
    value: State,
    receiver: Receiver<StateEvent>,
}

impl StateActor {
    pub fn new(receiver: Receiver<StateEvent>) -> Self {
        Self {
            value: State::new(),
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
                let selected_command = self.value.next_item();
                let _ = respond_to.send(selected_command);
            }
            StateEvent::SelectPreviousCommand { respond_to } => {
                let selected_command = self.value.previous_item();
                let _ = respond_to.send(selected_command);
            }
            StateEvent::ExecuteCommand => self.value.execute(),
            StateEvent::GetAllListItems { respond_to } => {
                let all_items = self.value.current_items.to_vec();
                let _ = respond_to.send(all_items);
            }
            StateEvent::CurrentCommand { respond_to } => {
                let selected_command = self.value.selected_command.clone();
                let _ = respond_to.send(selected_command);
            }
            StateEvent::PreviousTab { respond_to } => {
                let selected_namespace = self.value.previous_tab();
                let commands = self.value.current_items.clone();
                let selected_command = SelectedCommand::new(commands[0].clone(), 0);
                let _ = respond_to.send((selected_namespace, selected_command, commands));
            }
            StateEvent::NextTab { respond_to } => {
                let selected_namespace = self.value.next_tab();
                let commands = self.value.current_items.clone();
                let selected_command = SelectedCommand::new(commands[0].clone(), 0);
                let _ = respond_to.send((selected_namespace, selected_command, commands));
            }
            StateEvent::GetAllNamespaces { respond_to } => {
                let namespaces = self.value.namespaces.clone();
                let _ = respond_to.send(namespaces);
            }
        }
    }
}
