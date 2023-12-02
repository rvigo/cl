use super::{contexts::ui_context::UIContext, states::ui_state::ViewMode};
use crate::{
    entities::events::{app_events::AppEvent, input_events::InputMessages},
    key_handlers::{
        edit_handler::EditScreenHandler, help_popup_handler::HelpPopupHandler,
        insert_handler::InsertScreenHandler, main_handler::MainScreenHandler,
        popup_handler::PopupHandler, querybox_handler::QueryboxHandler, HandlerType,
        KeyEventHandler,
    },
};
use anyhow::{anyhow, Result};
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

pub struct InputHandler {
    input_rx: Receiver<InputMessages>,
    app_sx: Sender<AppEvent>,
    ui_context: Arc<Mutex<UIContext<'static>>>,
    should_quit: Arc<AtomicBool>,
    handlers: HashMap<HandlerType, ThreadSafeKeyEventHandler<'static>>,
}

impl InputHandler {
    pub async fn init(
        input_rx: Receiver<InputMessages>,
        app_sx: Sender<AppEvent>,
        ui_context: Arc<Mutex<UIContext<'static>>>,
        should_quit: Arc<AtomicBool>,
    ) -> Result<()> {
        let mut handlers: HashMap<HandlerType, ThreadSafeKeyEventHandler<'static>> = HashMap::new();

        handlers.insert(HandlerType::Popup, &PopupHandler);
        handlers.insert(HandlerType::Help, &HelpPopupHandler);
        handlers.insert(HandlerType::Main, &MainScreenHandler);
        handlers.insert(HandlerType::Insert, &InsertScreenHandler);
        handlers.insert(HandlerType::Edit, &EditScreenHandler);
        handlers.insert(HandlerType::QueryBox, &QueryboxHandler);

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
                InputMessages::KeyPress(key_event) => {
                    let event = self.handle_input(key_event)?;
                    if let Some(event) = event {
                        self.send_event(event).await?;
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

        let handler_type = self.get_handler_type(&ui_context);

        match self.handlers.get(&handler_type) {
            Some(handler) => handler.handle(key_event),
            None => Err(anyhow!("Key handler not found for {:?}", handler_type)),
        }
    }

    async fn send_event(&self, event: AppEvent) -> Result<(), SendError<AppEvent>> {
        debug!("sending event: {:?}", event);
        self.app_sx.send(event).await
    }

    fn get_handler_type(&self, ui_context: &UIContext<'static>) -> HandlerType {
        if ui_context.show_popup() {
            HandlerType::Popup
        } else if ui_context.show_help() {
            HandlerType::Help
        } else if ui_context.querybox_ref().focus() {
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
