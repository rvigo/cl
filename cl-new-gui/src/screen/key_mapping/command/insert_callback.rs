use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::InsertCommand;
use tokio::sync::mpsc::Sender;

pub enum InsertCallback {
    Save,
}

impl InsertCallback {
    pub async fn handle(self, state_tx: Sender<StateEvent>) {
        state_tx.send(InsertCommand).await.ok();
    }
}
