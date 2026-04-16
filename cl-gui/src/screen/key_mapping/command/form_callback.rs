use crate::screen::layer::FormMode;
use crate::state::state_event::StateEvent;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tracing::{debug, error};

pub enum FormCallback {
    Save(FormMode),
}

impl FormCallback {
    pub async fn handle(self, state_tx: Sender<StateEvent>) -> Result<(), String> {
        let (tx, rx) = oneshot::channel();

        let event = match self {
            FormCallback::Save(FormMode::Edit) => StateEvent::EditCommand { respond_to: tx },
            FormCallback::Save(FormMode::Insert) => StateEvent::InsertCommand { respond_to: tx },
        };

        if let Err(e) = state_tx.send(event).await {
            let err_msg = format!("FormCallback: failed to send state event: {e}");
            error!("{}", err_msg);
            return Err(err_msg);
        }

        match rx.await {
            Ok(Ok(_)) => {
                debug!("FormCallback: command saved successfully");
                Ok(())
            }
            Ok(Err(e)) => {
                let operation = match &self {
                    FormCallback::Save(FormMode::Edit) => "edit",
                    FormCallback::Save(FormMode::Insert) => "insert",
                };
                let msg = format!("Failed to {operation} command: {e}");
                error!("FormCallback: {}", msg);
                Err(msg)
            }
            Err(_) => {
                let err_msg = "FormCallback: response channel closed unexpectedly".to_string();
                error!("{}", err_msg);
                Err(err_msg)
            }
        }
    }
}
