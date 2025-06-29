use tokio::sync::mpsc::Sender;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::EditCommand;

pub enum EditCallback {
    Save,
    NextField,
    PreviousField,
}

impl EditCallback {
    pub async fn handle(self, state_tx: Sender<StateEvent>) {
        match self {
            EditCallback::Save => {
                state_tx.send(EditCommand).await.ok();
            }
            _ => {}
        }
    }
}
