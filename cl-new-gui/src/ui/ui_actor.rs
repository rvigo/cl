use crate::crossterm::{restore_terminal, setup_terminal};
use crate::state::state::SelectedCommand;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{GetAllListItems, GetAllNamespaces};
use crate::ui::ui::Ui;
use crate::ui::ui_event::UiEvent;
use anyhow::Result;
use cl_core::{Command, CommandVec};
use crossterm::event::EventStream;
use log::{error, info};
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_stream::StreamExt;

pub struct UiActor {
    ui: Ui,
    receiver: Receiver<UiEvent>,
}

const RENDERING_TICK_RATE: Duration = Duration::from_millis(250);

impl UiActor {
    pub fn new(receiver: Receiver<UiEvent>) -> Self {
        Self {
            ui: Ui::new(),
            receiver,
        }
    }

    pub fn handle_message(&mut self, message: UiEvent) {
        match message {
            UiEvent::ShowCommand(command) => {
                info!("showing command: {:?}", command);
            }
        }
    }

    async fn first_load(&mut self, state_tx: Sender<StateEvent>) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let event = GetAllListItems { respond_to: tx };

        state_tx.send(event).await.ok();

        let result = rx.await.ok();

        self.ui.update_list_items(result.aliases()).await;

        self.ui
            .select_command(SelectedCommand::new(result.first(), 0))
            .await;

        let (tx, rx) = tokio::sync::oneshot::channel();
        let event = GetAllNamespaces { respond_to: tx };

        state_tx.send(event).await.ok();
        let result = rx.await.ok();

        self.ui
            .update_tabs(result.clone().unwrap_or_default())
            .await;
    }

    pub async fn run(&mut self, state_tx: Sender<StateEvent>) -> Result<()> {
        let mut terminal = setup_terminal()?;
        let mut crossterm_events = EventStream::new();
        let mut ticker = tokio::time::interval(RENDERING_TICK_RATE);

        self.first_load(state_tx.clone()).await;

        let result: Result<()> = loop {
            tokio::select! {
                _ = ticker.tick() => (),
                event = crossterm_events.next() => {
                        self.ui.screens.handle_key_event(event, &state_tx).await;
                },

                message = self.receiver.recv() => {
                    if let Some(message) = message {
                        self.handle_message(message);
                    } else {
                        break Ok(());
                    }
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
