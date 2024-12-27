use crate::{
    context::UI,
    event::{AppEvent, InputEvent},
    key_handler::{
        EditScreenHandler, HandlerType, HelpPopupHandler, InsertScreenHandler, KeyEventHandler,
        MainScreenHandler, PopupHandler, QueryboxHandler,
    },
    register,
    widget::popup::Type,
    ViewMode,
};
use anyhow::{anyhow, Result};
use cl_core::hashmap;
use crossterm::event::KeyEvent;
use log::debug;
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::mpsc::{error::SendError, Receiver, Sender};

type ThreadSafeKeyEventHandler<'a> = &'a (dyn KeyEventHandler + Send + Sync);

pub struct InputEventHandler {
    input_rx: Receiver<InputEvent>,
    app_sx: Sender<AppEvent>,
    ui_context: Arc<Mutex<UI<'static>>>,
    should_quit: Arc<AtomicBool>,
    handlers: HashMap<HandlerType, ThreadSafeKeyEventHandler<'static>>,
}

impl InputEventHandler {
    pub async fn init(
        input_rx: Receiver<InputEvent>,
        app_sx: Sender<AppEvent>,
        ui_context: Arc<Mutex<UI<'static>>>,
        should_quit: Arc<AtomicBool>,
    ) -> Result<()> {
        let mut handlers: HashMap<HandlerType, ThreadSafeKeyEventHandler<'static>> = hashmap!();

        register!(
                handlers,
                HandlerType::Popup => &PopupHandler,
                HandlerType::Help => &HelpPopupHandler,
                HandlerType::Main => &MainScreenHandler,
                HandlerType::Insert => &InsertScreenHandler,
                HandlerType::Edit => &EditScreenHandler,
                HandlerType::QueryBox => &QueryboxHandler
        );

        let mut handler = Self {
            input_rx,
            app_sx,
            ui_context,
            should_quit,
            handlers,
        };

        handler.start().await
    }

    async fn start(&mut self) -> Result<()> {
        while let Some(message) = self.input_rx.recv().await {
            match message {
                InputEvent::KeyPress(key_event) => {
                    let event = self.handle_input(key_event)?;
                    if let Some(event) = event {
                        self.dispatch(event).await?;
                    }
                }
            };

            if self.should_quit.load(Ordering::SeqCst) {
                break;
            }
        }

        Ok(())
    }

    fn handle_input(&mut self, key_event: KeyEvent) -> Result<Option<AppEvent>> {
        let ui_context = self.ui_context.lock();

        let handler_type = self.get_handler(&ui_context);

        match self.handlers.get(&handler_type) {
            Some(handler) => handler.handle(key_event),
            None => Err(anyhow!("Key handler not found for {:?}", handler_type)),
        }
    }

    async fn dispatch(&self, event: AppEvent) -> Result<(), SendError<AppEvent>> {
        debug!("dispatching event: {:?}", event);
        self.app_sx.send(event).await
    }

    fn get_handler(&self, ui_context: &UI<'static>) -> HandlerType {
        if let Some(active_popup) = ui_context.popup.active_popup() {
            match active_popup.r#type {
                Type::Error | Type::Warning => HandlerType::Popup,
                Type::Help => HandlerType::Help,
            }
        } else if ui_context.querybox.focus() {
            HandlerType::QueryBox
        } else {
            match &ui_context.view_mode() {
                ViewMode::Main => HandlerType::Main,
                ViewMode::Insert => HandlerType::Insert,
                ViewMode::Edit => HandlerType::Edit,
            }
        }
    }
}
