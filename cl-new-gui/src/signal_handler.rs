use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum Signal {
    SigInt,
    UserInt,
}

pub struct SigHandler {
    signal_tx: broadcast::Sender<Signal>,
}

impl SigHandler {
    pub fn new(signal_tx: broadcast::Sender<Signal>) -> Self {
        Self { signal_tx }
    }

    pub fn send_signal(&mut self, signal: Signal) -> anyhow::Result<()> {
        self.signal_tx.send(signal)?;
        Ok(())
    }

    pub fn create() -> (SigHandler, broadcast::Receiver<Signal>) {
        let (tx, rx) = broadcast::channel(1);
        let sig_handler = SigHandler::new(tx);

        (sig_handler, rx)
    }
}
