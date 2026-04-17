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

    /// Explicit shutdown: sends `UserInt` to all subscribers and closes the
    /// channel.  Prefer this over dropping the handler silently so that
    /// subscribers receive the quit signal rather than a `Closed` error.
    pub fn shutdown(mut self) -> anyhow::Result<()> {
        self.send_signal(Signal::UserInt)
    }

    /// Creates a new `SignalHandler` together with a receiver that will be
    /// notified on every sent signal.
    ///
    /// The broadcast channel is sized at **8** to tolerate momentary lag in
    /// the receiver (e.g. while the UI is busy rendering).  With a buffer of
    /// 1 a signal could be silently dropped if the receiver hasn't polled yet.
    pub fn create() -> (SignalHandler, broadcast::Receiver<Signal>) {
        let (tx, rx) = broadcast::channel(8);
        let sig_handler = SignalHandler::new(tx);

        (sig_handler, rx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shutdown_sends_user_int_signal() {
        let (handler, mut rx) = SignalHandler::create();
        handler.shutdown().expect("shutdown should succeed");
        match rx.try_recv() {
            Ok(Signal::UserInt) => {}
            other => panic!("expected UserInt signal, got {:?}", other),
        }
    }

    #[test]
    fn send_signal_returns_error_with_no_active_receivers() {
        let (tx, rx) = broadcast::channel::<Signal>(8);
        drop(rx); // no receivers
        let mut handler = SignalHandler::new(tx);
        assert!(
            handler.send_signal(Signal::SigInt).is_err(),
            "send should fail when there are no receivers"
        );
    }

    #[test]
    fn lagged_receiver_misses_overflowed_messages() {
        // Use a channel with capacity 1 to force lag with 2 consecutive sends
        let (tx, mut rx) = broadcast::channel::<Signal>(1);
        let mut handler = SignalHandler::new(tx);
        let _ = handler.send_signal(Signal::SigInt);
        let _ = handler.send_signal(Signal::SigInt); // overflows capacity
        match rx.try_recv() {
            Err(broadcast::error::TryRecvError::Lagged(_)) => {}
            other => panic!("expected Lagged error, got {:?}", other),
        }
    }
}
