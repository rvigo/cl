use super::{
    application_context::ApplicationContext,
    events::app_events::{AppEvents, CommandEvents, RenderEvents},
    ui_state::{UiState, ViewMode},
};
use log::debug;
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::mpsc::Receiver;

pub struct AppRouter<'a> {
    app_rx: Receiver<AppEvents>,
    context: Arc<Mutex<ApplicationContext<'a>>>,
    ui_state: Arc<Mutex<UiState>>,

    should_quit: Arc<AtomicBool>,
}

impl<'a> AppRouter<'a> {
    pub async fn init(
        app_rx: Receiver<AppEvents>,
        context: Arc<Mutex<ApplicationContext<'a>>>,
        ui_state: Arc<Mutex<UiState>>,

        should_quit: Arc<AtomicBool>,
    ) {
        let mut app_router = Self {
            app_rx,
            context,
            ui_state,
            should_quit,
        };

        app_router.start().await
    }

    async fn start(&mut self) {
        while let Some(message) = self.app_rx.recv().await {
            match message {
                AppEvents::Run(command_event) => match command_event {
                    CommandEvents::Execute(command) => {
                        self.context.lock().set_callback_command(command);
                        self.quit()
                    }
                    CommandEvents::Insert(command) => {
                        self.context.lock().add_command(command);
                        self.ui_state.lock().view_mode = ViewMode::Main;
                    }
                    CommandEvents::Delete(_) => todo!(),
                    CommandEvents::Edit {
                        edited_command,
                        old_command,
                    } => {
                        self.context
                            .lock()
                            .add_edited_command(edited_command, old_command);
                        self.ui_state.lock().view_mode = ViewMode::Main;
                    }
                },
                AppEvents::Render(render_events) => match render_events {
                    RenderEvents::Main => {
                        self.ui_state.lock().view_mode = ViewMode::Main;
                        self.context.lock().enter_main_mode();
                    }
                    RenderEvents::Edit => {
                        self.ui_state.lock().view_mode = ViewMode::Edit;
                        self.context.lock().enter_edit_mode();
                    }
                    RenderEvents::Insert => {
                        self.ui_state.lock().view_mode = ViewMode::Insert;
                        self.context.lock().enter_insert_mode();
                    }
                },
                AppEvents::Quit => self.quit(),
            };
        }
    }

    fn quit(&mut self) {
        debug!("quitting app");
        self.should_quit.store(true, Ordering::SeqCst)
    }
}
