use super::{contexts::ui_context::UIContext, states::ui_state::ViewMode};
use crate::gui::{
    entities::events::{app_events::AppEvent, input_events::InputMessages},
    key_handlers::{
        edit_handler::EditScreenHandler, help_popup_handler::HelpPopupHandler,
        insert_handler::InsertScreenHandler, main_handler::MainScreenHandler,
        popup_handler::PopupHandler, querybox_handler::QueryboxHandler, KeyEventHandler,
    },
};
use anyhow::Result;
use crossterm::event::KeyEvent;
use log::debug;
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc::{error::SendError, Receiver, Sender};

pub struct InputHandler {
    input_rx: Receiver<InputMessages>,
    app_sx: Sender<AppEvent>,
    ui_context: Arc<Mutex<UIContext<'static>>>,
    should_quit: Arc<AtomicBool>,
    main_screen_handler: MainScreenHandler,
    insert_screen_handler: InsertScreenHandler,
    edit_screen_handler: EditScreenHandler,
    popup_handler: PopupHandler,
    help_popup_handler: HelpPopupHandler,
    querybox_handler: QueryboxHandler,
}

impl InputHandler {
    pub async fn init(
        input_rx: Receiver<InputMessages>,
        app_sx: Sender<AppEvent>,
        ui_context: Arc<Mutex<UIContext<'static>>>,
        should_quit: Arc<AtomicBool>,
    ) -> Result<()> {
        let mut handler = Self {
            input_rx,
            app_sx,
            ui_context,
            should_quit,
            // key handlers
            main_screen_handler: MainScreenHandler,
            insert_screen_handler: InsertScreenHandler,
            edit_screen_handler: EditScreenHandler,
            popup_handler: PopupHandler::new(None),
            help_popup_handler: HelpPopupHandler,
            querybox_handler: QueryboxHandler,
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

        let handled_event = if ui_context.show_popup() {
            self.popup_handler
                .update_message_type(ui_context.popup().and_then(|popup| popup.message_type()));
            self.popup_handler.handle(key_event)
        } else if ui_context.show_help() {
            self.help_popup_handler.handle(key_event)
        } else if ui_context.querybox_focus() {
            self.querybox_handler.handle(key_event)
        } else {
            let handler = self.get_handler(&ui_context.view_mode());
            handler.handle(key_event)
        };

        handled_event
    }

    async fn send_event(&self, event: AppEvent) -> Result<(), SendError<AppEvent>> {
        debug!("sending event: {:?}", event);
        self.app_sx.send(event).await
    }

    fn get_handler(&self, view_mode: &ViewMode) -> &dyn KeyEventHandler {
        match view_mode {
            ViewMode::Main => &self.main_screen_handler,
            ViewMode::Insert => &self.insert_screen_handler,
            ViewMode::Edit => &self.edit_screen_handler,
        }
    }
}
