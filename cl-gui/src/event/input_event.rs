use crate::impl_event;
use crate::sync_cell::SyncCell;
use crossterm::event::KeyEvent;
use tokio::sync::mpsc;

static TX: SyncCell<mpsc::UnboundedSender<InputEvent>> = SyncCell::new();
static RX: SyncCell<mpsc::UnboundedReceiver<InputEvent>> = SyncCell::new();

#[derive(Clone, Debug)]
pub enum InputEvent {
    KeyPress(KeyEvent),
}

impl_event!(InputEvent);
