use crate::component::List;
use crate::crossterm::{restore_terminal, setup_terminal};
use crate::observer::{ObserverEvent, Publisher};
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{
    ExecuteCommand, GetAllItems, SelectNextCommand, SelectPreviousCommand,
};
use crate::ui::ui::Ui;
use crate::ui::ui_actor::KeyHandleResult::{Close, Continue};
use crate::ui::ui_event::UiEvent;
use anyhow::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
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
        let event = GetAllItems { respond_to: tx };

        if let Err(e) = state_tx.send(event).await {
            eprintln!("Failed to send state event: {:?}", e);
            return;
        }

        let result = match rx.await {
            Ok(data) => Some(data),
            Err(e) => {
                eprintln!("Failed to receive items: {:?}", e);
                None
            }
        };

        let items = result
            .as_ref()
            .map(|data| {
                data.iter()
                    .map(|item| item.alias.to_string())
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default();

        self.ui.screens.main.list = List::new(items);

        if let Some(first_item) = result.and_then(|data| data.first().cloned()) {
            self.ui
                .screens
                .get_active_screen_mut()
                .get_publisher()
                .notify(ObserverEvent::new(first_item.clone()))
                .await;
        } else {
            eprintln!("No items available to update components");
        }
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
                            if let Close = self.handle_key_events(event, &state_tx).await {
                            break Ok(())
                        }
                },

                message = self.receiver.recv() => {
                    if let Some(message) = message {
                        self.handle_message(message);
                    } else {
                        break Ok(());
                    }
                }
            }

            if let Err(err) =
                terminal.draw(|frame| self.ui.screens.get_active_screen_mut().render(frame))
            {
                error!("an error occurred: {err}")
            }
        };

        restore_terminal(&mut terminal)?;

        result
    }

    async fn handle_key_events(
        &mut self,
        key_event: Option<std::io::Result<Event>>,
        state_tx: &Sender<StateEvent>,
    ) -> KeyHandleResult {
        match key_event {
            Some(Ok(Event::Key(key))) => {
                match key {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    } => Close,
                    KeyEvent {
                        code: KeyCode::Char('j'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    } => {
                        // If the user press 's' key, we will select a command and send an event to state_actor.
                        // Also, an oneshot channel will be created:
                        // - TX: the state_actor will respond with the selected command
                        // - RX: the ui_actor will receive the selected command and update the ui
                        let (tx, rx) = tokio::sync::oneshot::channel();
                        let event = SelectNextCommand { respond_to: tx };

                        state_tx.send(event).await.ok();
                        let maybe_cmd = rx.await.ok();
                        if let Some(selected_command) = maybe_cmd {
                            self.ui.next_command(selected_command).await;
                        }

                        Continue
                    }
                    KeyEvent {
                        code: KeyCode::Char('k'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    } => {
                        let (tx, rx) = tokio::sync::oneshot::channel();
                        let event = SelectPreviousCommand { respond_to: tx };

                        state_tx.send(event).await.ok();
                        let maybe_cmd = rx.await.ok();
                        if let Some(selected_command) = maybe_cmd {
                            self.ui.previous_command(selected_command).await;
                        }

                        Continue
                    }
                    KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: KeyModifiers::NONE,
                        ..
                    } => {
                        let event = ExecuteCommand;
                        state_tx.send(event).await.ok();

                        Continue
                    }
                    _ => Continue,
                }
            }
            None => Continue,
            _ => Continue,
        }
    }
}

enum KeyHandleResult {
    Continue,
    Close,
}
