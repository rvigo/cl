use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum Signal {
    SigInt,
    UserInt,
}

pub struct SignalHandler {
    signal_tx: broadcast::Sender<Signal>,
}

impl SignalHandler {
    pub fn new(signal_tx: broadcast::Sender<Signal>) -> Self {
        Self { signal_tx }
    }

    pub fn send_signal(&mut self, signal: Signal) -> anyhow::Result<()> {
        self.signal_tx.send(signal)?;
        Ok(())
    }

    pub fn create() -> (SignalHandler, broadcast::Receiver<Signal>) {
        let (tx, rx) = broadcast::channel(1);
        let sig_handler = SignalHandler::new(tx);

        (sig_handler, rx)
    }
}
