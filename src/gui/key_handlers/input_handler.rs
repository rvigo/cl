use super::{popup_handler, query_box_handler};
use crate::gui::entities::events::app_events::AppEvents;
use crate::gui::entities::events::input_events::InputMessages;
use crate::gui::{
    entities::{ui_context::UIContext, ui_state::ViewMode},
    key_handlers::{edit_handler, insert_handler, main_handler},
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
    ui_context: Arc<Mutex<UIContext<'static>>>,
    should_quit: Arc<AtomicBool>,
}

impl InputHandler {
    pub async fn init(
        input_rx: Receiver<InputMessages>,
        app_sx: Sender<AppEvents>,
        ui_context: Arc<Mutex<UIContext<'static>>>,
        should_quit: Arc<AtomicBool>,
    ) -> Result<()> {
        let mut handler = Self {
            input_rx,
            app_sx,
            ui_context,
            should_quit,
        };

        handler.start().await
    }

    async fn start(&mut self) -> Result<()> {
        while let Some(message) = self.input_rx.recv().await {
            match message {
                InputMessages::KeyPress(key_event) => self.handle_input(key_event).await?,
            };

            if self.should_quit.load(Ordering::SeqCst) {
                break;
            }
        }

        Ok(())
    }

    async fn handle_input(&mut self, key_event: KeyEvent) -> Result<()> {
        let ui_context = self.ui_context.lock().to_owned();
        let result = if ui_context.show_popup() {
            popup_handler::handle(key_event, &mut self.ui_context)?
        } else if ui_context.show_help() {
            popup_handler::handle_help()?
        } else if ui_context.querybox_focus() {
            query_box_handler::handle(key_event, &mut self.ui_context)?
        } else {
            match ui_context.view_mode() {
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
