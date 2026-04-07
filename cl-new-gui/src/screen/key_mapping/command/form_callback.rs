use crate::screen::layer::FormMode;
use crate::state::state_event::StateEvent;
use tokio::sync::mpsc::Sender;

pub enum FormCallback {
    Save(FormMode),
}

impl FormCallback {
    pub async fn handle(self, state_tx: Sender<StateEvent>) {
        match self {
            FormCallback::Save(FormMode::Edit) => {
                state_tx.send(StateEvent::EditCommand).await.ok();
            }
            FormCallback::Save(FormMode::Insert) => {
                state_tx.send(StateEvent::InsertCommand).await.ok();
            }
        }
    }
}
