use crate::state::state_actor::StateActor;
use crate::state::state_event::StateEvent;
use cl_core::Command;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct StateHandler {
    sender: Sender<StateEvent>,
}

impl StateHandler {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(8);
        let mut actor = StateActor::new(rx);

        tokio::spawn(async move {
            actor.run().await;
        });

        Self { sender: tx }
    }

    pub async fn select_command(&self, idx: usize) -> Option<Command<'static>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.sender
            .send(StateEvent::SelectNextCommand { respond_to: tx })
            .await
            .ok()?;
        rx.await.ok()
    }
}
