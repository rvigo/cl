use crate::{
    context::UI,
    event::{AppEvent, InputEvent},
    key_handler::{
        EditScreenHandler, HandlerType, HelpPopupHandler, InsertScreenHandler, KeyEventHandler,
        MainScreenHandler, PopupHandler, QueryboxHandler,
    },
    widget::popup::Type,
    ViewMode,
};
use anyhow::Result;
use crossterm::event::KeyEvent;
use log::debug;
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc::UnboundedReceiver;

type ThreadSafeKeyEventHandler<'a> = &'a (dyn KeyEventHandler + Send + Sync);

pub struct InputEventHandler {
    ui_context: Arc<Mutex<UI<'static>>>,
    should_quit: Arc<AtomicBool>,
}

impl InputEventHandler {
    pub fn init(ui_context: Arc<Mutex<UI<'static>>>, should_quit: Arc<AtomicBool>) -> Self {
        Self {
            ui_context,
            should_quit,
        }
    }

    pub async fn handle(self, mut rx: UnboundedReceiver<InputEvent>) -> Result<()> {
        while let Some(message) = rx.recv().await {
            match message {
                InputEvent::KeyPress(key_event) => {
                    let event = Self::handle_input(key_event, &self.ui_context.lock())?;
                    if let Some(event) = event {
                        event.emit()
                    }
                }
            };

            if self.should_quit.load(Ordering::SeqCst) {
                break;
            }
        }

        Ok(())
    }

    fn handle_input(key_event: KeyEvent, ui_context: &UI) -> Result<Option<AppEvent>> {
        let handler_type = Self::get_handler_type(ui_context);
        let handler: ThreadSafeKeyEventHandler<'static> = match handler_type {
            HandlerType::Popup => &PopupHandler,
            HandlerType::Help => &HelpPopupHandler,
            HandlerType::Main => &MainScreenHandler,
            HandlerType::Insert => &InsertScreenHandler,
            HandlerType::Edit => &EditScreenHandler,
            HandlerType::QueryBox => &QueryboxHandler,
        };

        handler.handle(key_event)
    }

    fn get_handler_type(ui_context: &UI<'_>) -> HandlerType {
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
