use crate::crossterm::{restore_terminal, setup_terminal};
use crate::state::state_event::StateEvent;
use crate::state::state_event::StateEvent::{ExecuteCommand, SelectNextCommand};
use crate::ui::ui::Ui;
use crate::ui::ui_event::UiEvent;
use anyhow::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
use log::{debug, error, info};
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
            ui: Ui::default(),
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

    pub async fn run(&mut self, state_tx: Sender<StateEvent>) -> Result<()> {
        let mut terminal = setup_terminal()?;
        let mut crossterm_events = EventStream::new();
        let mut ticker = tokio::time::interval(RENDERING_TICK_RATE);

        let result: Result<()> = loop {
            tokio::select! {
                    _ = ticker.tick() => (),
                event = crossterm_events.next() => self.handle_key_events(event, &state_tx).await?,

                message = self.receiver.recv() => {
                    if let Some(message) = message {
                        self.handle_message(message);
                    } else {
                        break Ok(());
                    }
                }
            }

            if let Err(err) =
                terminal.draw(|frame| self.ui.screens.get_active_screen().render(frame))
            {
                error!("an error occurred: {err}")
            }
        };

        restore_terminal(&mut terminal)?;

        result
    }

    async fn handle_key_events(&mut self, key_event: Option<std::io::Result<Event>>, state_tx: &Sender<StateEvent>) -> Result<()> {
        match key_event {
            Some(Ok(Event::Key(key))) => {
                match key {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: KeyModifiers::NONE,
                        ..
                    } => Ok(()),
                    KeyEvent {
                        code: KeyCode::Char('s'),
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
                        self.ui.select_command(maybe_cmd.clone()).await;
                        Ok(())
                    }
                    KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: KeyModifiers::NONE,
                        ..
                    } => {
                        if let Some(cmd) = &self.ui.selected_command {
                            debug!("sending command: {:#?}", cmd);
                            let event = ExecuteCommand(cmd.clone());
                            state_tx.send(event).await.ok();
                        }

                        Ok(())
                    }
                    _ => Ok(())
                }
            }
            None => Ok(()),
            _ => Ok(())
        }
    }
}
