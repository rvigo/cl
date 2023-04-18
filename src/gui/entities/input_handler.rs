use super::{contexts::ui_context::UIContext, states::ui_state::ViewMode};
use crate::gui::{
    entities::events::{app_events::AppEvent, input_events::InputMessages},
    key_handlers::{
        edit_handler::EditScreenHandler, help_popup_handler::HelpPopupHandler,
        insert_handler::InsertScreenHandler, main_handler::MainScreenHandler,
        popup_handler::PopupHandler, querybox_handler::QueryboxHandler, KeyEventHandler,
        WidgetKeyEventHandler,
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
use tokio::sync::mpsc::{Receiver, Sender};

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
            popup_handler: PopupHandler,
            help_popup_handler: HelpPopupHandler,
            querybox_handler: QueryboxHandler,
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
            self.popup_handler
                .handle(key_event, &mut self.ui_context.lock())?
        } else if ui_context.show_help() {
            self.help_popup_handler.handle(key_event)?
        } else if ui_context.querybox_focus() {
            self.querybox_handler
                .handle(key_event, &mut self.ui_context.lock())?
        } else {
            let handler = self.get_handler(&ui_context.view_mode());
            handler.handle(key_event)?
        };

        if let Some(event) = result {
            debug!("sending event: {:?}", event);
            self.app_sx.send(event).await?;
        }

        Ok(())
    }

    fn get_handler(&self, view_mode: &ViewMode) -> &dyn KeyEventHandler {
        match view_mode {
            ViewMode::Main => &self.main_screen_handler,
            ViewMode::Insert => &self.insert_screen_handler,
            ViewMode::Edit => &self.edit_screen_handler,
        }
    }
}