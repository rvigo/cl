use crate::screen::layer::FormMode;
use crate::state::state_event::StateEvent;
use tokio::sync::mpsc::Sender;

pub enum FormCallback {
    Save(FormMode),
}

impl FormCallback {
    pub async fn handle(self, state_tx: Sender<StateEvent>) {
        let event = match self {
            FormCallback::Save(FormMode::Edit) => StateEvent::EditCommand,
            FormCallback::Save(FormMode::Insert) => StateEvent::InsertCommand,
        };
        if let Err(e) = state_tx.send(event).await {
            tracing::error!("FormCallback: failed to send state event: {e}");
        }
    }
}
