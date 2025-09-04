use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::EditCommand;
use tokio::sync::mpsc::Sender;

pub enum EditCallback {
    Save,
    NextField,
    PreviousField,
}

impl EditCallback {
    pub async fn handle(self, state_tx: Sender<StateEvent>) {
        if let EditCallback::Save = self {
            state_tx.send(EditCommand).await.ok();
        }
    }
}
