use crate::state::state::State;
use crate::state::state_event::StateEvent;
use anyhow::Result;
use cl_core::CommandExec;
use log::debug;
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

    pub fn handle_message(&mut self, message: StateEvent) {
        match message {
            StateEvent::SelectNextCommand { respond_to } => {
                let selected_command = self.value.next();
                debug!("responding selected command: {:?}", selected_command);
                let _ = respond_to.send(selected_command);
            }
            StateEvent::ExecuteCommand(cmd) => { cmd.exec(true, false).ok(); } ,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        while let Some(message) = self.receiver.recv().await {
            self.handle_message(message);
        }

        Ok(())
    }
}
