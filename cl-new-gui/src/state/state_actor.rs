use crate::state::state::State;
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
                let selected_command = self.value.next();
                let _ = respond_to.send(selected_command);
            }
            StateEvent::SelectPreviousCommand { respond_to } => {
                let selected_command = self.value.previous();
                let _ = respond_to.send(selected_command);
            }
            StateEvent::ExecuteCommand => self.value.execute(),
            StateEvent::GetAllItems { respond_to } => {
                let all_items = self.value.commands.as_list();
                let _ = respond_to.send(all_items);
            }
            StateEvent::CurrentCommand { respond_to } => {
                let selected_command = self.value.selected_command.clone();
                let _ = respond_to.send(selected_command);
            }
        }
    }
}
