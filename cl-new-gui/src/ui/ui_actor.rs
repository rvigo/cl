use crate::crossterm::{restore_terminal, setup_terminal};
use crate::oneshot;
use crate::signal_handler::{SignalHandler, Signal};
use crate::state::selected_command::SelectedCommand;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{GetAllListItems, GetAllNamespaces};
use crate::ui::Ui;
use anyhow::Result;
use cl_core::{CommandBuilder, CommandVecExt};
use crossterm::event::EventStream;
use tracing::{debug, error};
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::sync::mpsc::Sender;
use tokio_stream::StreamExt;

pub struct UiActor {
    ui: Ui,
    signal_handler: SignalHandler,
    signal_receiver: broadcast::Receiver<Signal>,
}

const RENDERING_TICK_RATE: Duration = Duration::from_millis(250);

impl Default for UiActor {
    fn default() -> Self {
        Self::new()
    }
}

impl UiActor {
    pub fn new() -> Self {
        let (sig_handler, receiver) = SignalHandler::create();

        Self {
            ui: Ui::default(),
            signal_handler: sig_handler,
            signal_receiver: receiver,
        }
    }

    /// Initial load of the UI
    async fn initial_load(&mut self, state_tx: Sender<StateEvent>) {
        let result = oneshot!(state_tx, GetAllListItems).ok();
        self.ui
            .update_list_items(result.as_ref().map_or_else(Vec::new, |vec| vec.aliases()))
            .await;
        self.ui
            .select_command(SelectedCommand::new(
                result.as_ref().unwrap_or(&vec![]).first()
                    .unwrap_or_else(|| CommandBuilder::default().build()),
                0,
            ))
            .await;

        let result = oneshot!(state_tx, GetAllNamespaces).ok();
        self.ui
            .update_tabs(result.clone().unwrap_or_default())
            .await;
    }

    pub async fn run(&mut self, state_tx: Sender<StateEvent>) -> Result<()> {
        let mut terminal = setup_terminal()?;
        let mut crossterm_events = EventStream::new();
        let mut ticker = tokio::time::interval(RENDERING_TICK_RATE);

        self.initial_load(state_tx.clone()).await;

        let result: Result<()> = loop {
            tokio::select! {
                // ticker
                _ = ticker.tick() => (),
                // key event
                event = crossterm_events.next() => {
                        self.ui.screens.handle_key_event(event, &state_tx, &mut self.signal_handler).await;
                },
                // Quit signal
                Ok(message) = self.signal_receiver.recv() => {
                    debug!("Received signal: {:?}", message);
                    break Ok(())
                }
            }

            if let Err(err) = terminal.draw(|frame| self.ui.screens.render_layers(frame)) {
                error!("an error occurred: {err}")
            }
        };

        restore_terminal(&mut terminal)?;

        result
    }
}
