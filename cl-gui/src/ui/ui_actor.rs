use crate::component::{List, Tabs, TextBox};
use crate::crossterm::{restore_terminal, setup_terminal};
use crate::observer::event::{Event, ListEvent, TabsEvent, TextBoxEvent};
use crate::screen::Screen;
use crate::signal_handler::{Signal, SignalHandler};
use crate::state::selected_command::SelectedCommand;
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{GetAllListItems, GetAllNamespaces};
use anyhow::Result;
use cl_core::CommandVecExt;
use crossterm::event::EventStream;
use std::any::TypeId;
use tokio::sync::broadcast;
use tokio::sync::mpsc::Sender;
use tokio_stream::StreamExt;
use tracing::{debug, error};

pub struct UiActor {
    screen: Screen,
    signal_handler: SignalHandler,
    signal_receiver: broadcast::Receiver<Signal>,
}

impl Default for UiActor {
    fn default() -> Self {
        Self::new()
    }
}

impl UiActor {
    pub fn new() -> Self {
        let (sig_handler, receiver) = SignalHandler::create();

        Self {
            screen: Screen::new(),
            signal_handler: sig_handler,
            signal_receiver: receiver,
        }
    }

    /// Initial load of the UI
    async fn initial_load(&mut self, state_tx: Sender<StateEvent>) {
        let result = oneshot!(state_tx, GetAllListItems).ok();

        self.screen
            .notify(
                TypeId::of::<List>(),
                Event::List(ListEvent::UpdateAll(
                    result.as_ref().map_or_else(Vec::new, |vec| vec.aliases()),
                )),
            )
            .await;

        // Populate navigation snapshot for UI-local j/k navigation
        if let Some(items) = &result {
            self.screen.set_snapshot(items.clone(), 0);
        }

        if let Some(cmd) = result.as_ref().and_then(|vec| vec.first()) {
            let selected = SelectedCommand::new(cmd.clone(), 0);
            self.screen
                .notify(
                    TypeId::of::<TextBox>(),
                    Event::TextBox(TextBoxEvent::UpdateCommand(selected.value.clone())),
                )
                .await;
        }

        let result = oneshot!(state_tx, GetAllNamespaces).ok();
        self.screen
            .notify(
                TypeId::of::<Tabs>(),
                Event::Tabs(TabsEvent::UpdateAll(result.unwrap_or_default())),
            )
            .await;
    }

    pub async fn run(&mut self, state_tx: Sender<StateEvent>) -> Result<()> {
        let mut terminal = setup_terminal()?;
        let mut crossterm_events = EventStream::new();

        self.initial_load(state_tx.clone()).await;

        let result: Result<()> = loop {
            tokio::select! {
                // Signal branch is checked first so a quit is never starved
                // by rapid key input.  `biased` preserves the declaration
                // order instead of rotating randomly.
                biased;

                // Quit signal — handle RecvError::Lagged as a quit too:
                // if we lag behind on signals the channel dropped a message,
                // which means a quit was missed; treat that as an exit.
                signal_result = self.signal_receiver.recv() => {
                    match signal_result {
                        Ok(message) => {
                            debug!("Received signal: {:?}", message);
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                            debug!("Signal channel lagged by {n} messages; treating as quit");
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                            debug!("Signal channel closed");
                        }
                    }
                    break Ok(())
                }

                // key / resize event — render immediately after handling
                event = crossterm_events.next() => {
                    self.screen.handle_key_event(event, &state_tx, &mut self.signal_handler).await;
                },
            }

            if let Err(err) = terminal.draw(|frame| self.screen.render_layers(frame)) {
                error!("an error occurred: {err}")
            }
        };

        restore_terminal(&mut terminal)?;

        result
    }
}
