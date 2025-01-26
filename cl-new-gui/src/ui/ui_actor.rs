use crate::crossterm::{restore_terminal, setup_terminal};
use crate::signal_handler::{SigHandler, Signal};
use crate::state::state::SelectedCommand;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{GetAllListItems, GetAllNamespaces};
use crate::ui::ui::Ui;
use anyhow::Result;
use cl_core::{Command, CommandVec};
use crossterm::event::EventStream;
use log::{debug, error};
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::sync::mpsc::Sender;
use tokio_stream::StreamExt;
use crate::oneshot;

pub struct UiActor {
    ui: Ui,
    sig_handler: SigHandler, // TODO rethink this name
    sig_receiver: broadcast::Receiver<Signal>,
}

const RENDERING_TICK_RATE: Duration = Duration::from_millis(250);

impl UiActor {
    pub fn new() -> Self {
        let (sig_handler, receiver) = SigHandler::create();

        Self {
            ui: Ui::new(),
            sig_handler,
            sig_receiver: receiver,
        }
    }

    /// Initial load of the UI
    async fn initial_load(&mut self, state_tx: Sender<StateEvent>) {
        let result = oneshot!(state_tx, GetAllListItems);
        self.ui.update_list_items(result.aliases()).await;
        self.ui
            .select_command(SelectedCommand::new(result.first(), 0))
            .await;

        let result = oneshot!(state_tx, GetAllNamespaces);
        self.ui
            .update_tabs(result.clone().unwrap_or_default())
            .await;
    }

    pub async fn run(
        &mut self,
        state_tx: Sender<StateEvent>,
    ) -> Result<()> {
        let mut terminal = setup_terminal()?;
        let mut crossterm_events = EventStream::new();
        let mut ticker = tokio::time::interval(RENDERING_TICK_RATE);

        self.initial_load(state_tx.clone()).await;

        let result: Result<()> = loop {
            tokio::select! {
                // ticker
                _ = ticker.tick() => (),
                // crossterm key event 
                event = crossterm_events.next() => {
                        self.ui.screens.handle_key_event(event, &state_tx, &mut self.sig_handler).await;
                },
                // Quit signal
                Ok(message) = self.sig_receiver.recv() => {
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

pub trait CommandVecExt {
    fn aliases(&self) -> Vec<String>;

    fn namespaces(&self) -> Vec<String>;

    fn first(&self) -> Command<'static>;
}

impl CommandVecExt for CommandVec<'static> {
    fn aliases(&self) -> Vec<String> {
        self.iter().map(|cmd| cmd.alias.to_string()).collect()
    }

    fn namespaces(&self) -> Vec<String> {
        self.iter().map(|cmd| cmd.namespace.to_string()).collect()
    }

    fn first(&self) -> Command<'static> {
        self.get(0)
            .cloned()
            .expect("List is empty, cannot retrieve the first element")
    }
}

impl CommandVecExt for Option<CommandVec<'static>> {
    fn aliases(&self) -> Vec<String> {
        self.as_ref().map_or_else(Vec::new, |vec| vec.aliases())
    }

    fn namespaces(&self) -> Vec<String> {
        self.as_ref().map_or_else(Vec::new, |vec| vec.namespaces())
    }

    fn first(&self) -> Command<'static> {
        self.as_ref().unwrap_or(&vec![]).first()
    }
}
