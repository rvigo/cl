use super::{popup_handler, query_box_handler};
use crate::{
    gui::{
        entities::{
            application_context::ApplicationContext,
            ui_state::{UiState, ViewMode},
        },
        key_handlers::{edit_handler, insert_handler, main_handler},
    },
    AppEvents, InputMessages,
};
use anyhow::Result;
use crossterm::event::KeyEvent;
use log::debug;
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc::{Receiver, Sender};

pub struct InputHandler {
    input_rx: Receiver<InputMessages>,
    app_sx: Sender<AppEvents>,
    ui_state: Arc<Mutex<UiState>>,
    should_quit: Arc<AtomicBool>,
    context: Arc<Mutex<ApplicationContext<'static>>>,
}

impl InputHandler {
    pub async fn init(
        input_rx: Receiver<InputMessages>,
        app_sx: Sender<AppEvents>,
        ui_state: Arc<Mutex<UiState>>,
        should_quit: Arc<AtomicBool>,
        context: Arc<Mutex<ApplicationContext<'static>>>,
    ) -> Result<()> {
        let mut handler = Self {
            input_rx,
            app_sx,
            ui_state,
            should_quit,
            context,
        };

        handler.start().await
    }

    async fn start(&mut self) -> Result<()> {
        while let Some(message) = self.input_rx.recv().await {
            match message {
                InputMessages::KeyPress(key_event) => self.handle_input(key_event).await?,
            };

            if self.should_quit.load(Ordering::SeqCst) {
                debug!("quiting input handler");
                break;
            }
        }

        Ok(())
    }

    async fn handle_input(&mut self, key_event: KeyEvent) -> Result<()> {
        let ui_state = self.ui_state.lock().to_owned();
        let result = if ui_state.show_popup.load(Ordering::SeqCst) {
            popup_handler::handle(key_event, &mut self.context)?
        } else if ui_state.show_help.load(Ordering::SeqCst) {
            popup_handler::handle_help()?
        } else if ui_state.query_box_active.load(Ordering::SeqCst) {
            query_box_handler::handle(key_event, &mut self.context)?
        } else {
            match ui_state.view_mode {
                ViewMode::Main => main_handler::handle(key_event)?,
                ViewMode::Insert => insert_handler::handle(key_event)?,
                ViewMode::Edit => edit_handler::handle(key_event)?,
            }
        };

        if let Some(event) = result {
            debug!("sending event: {:?}", event);
            self.app_sx.send(event).await?;
        }
        Ok(())
    }
}
