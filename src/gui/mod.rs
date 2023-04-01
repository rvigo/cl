mod entities;
mod key_handlers;
mod layouts;
mod widgets;

pub mod core {
    use anyhow::Result;
    use log::debug;
    use parking_lot::Mutex;
    use std::sync::{atomic::AtomicBool, Arc};
    use tokio::sync::mpsc::{Receiver, Sender};

    use crate::{
        gui::{
            entities::{
                application_context::ApplicationContext,
                event_listener::EventListener,
                events::{app_events::AppEvents, input_events::InputMessages},
                tui_application::TuiApplication,
                ui_context::UIContext,
            },
            key_handlers::input_handler::InputHandler,
        },
        resources::{config::Config, file_service::FileService},
    };

    pub async fn init(config: Config) -> Result<()> {
        debug!("creating channels");
        let (app_sx, app_rx) = tokio::sync::mpsc::channel::<AppEvents>(32);
        let (input_sx, input_rx) = tokio::sync::mpsc::channel::<InputMessages>(32);

        debug!("loading commands");
        let file_service = FileService::new(config.get_command_file_path()?);
        let commands = file_service.load_commands_from_file()?;

        debug!("creating context");
        let should_quit: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let ui_context = Arc::new(Mutex::new(UIContext::new()));
        let context = Arc::new(Mutex::new(ApplicationContext::init(
            commands,
            file_service,
            config.get_options(),
        )));

        debug!("starting components");
        handler_init(input_rx, &app_sx, &ui_context, &should_quit).await;
        event_listener_init(app_rx, &context, &ui_context, &should_quit).await;
        ui_init(input_sx, should_quit, ui_context, context).await?;

        Ok(())
    }

    async fn event_listener_init(
        app_rx: Receiver<AppEvents>,
        context: &Arc<Mutex<ApplicationContext>>,
        ui_context: &Arc<Mutex<UIContext<'static>>>,
        should_quit: &Arc<AtomicBool>,
    ) {
        debug!("starting event listener");
        tokio::spawn(EventListener::init(
            app_rx,
            context.clone(),
            ui_context.clone(),
            should_quit.clone(),
        ));
    }

    async fn ui_init<'a>(
        input_sx: Sender<InputMessages>,
        should_quit: Arc<AtomicBool>,
        ui_state: Arc<Mutex<UIContext<'static>>>,
        context: Arc<Mutex<ApplicationContext>>,
    ) -> Result<()> {
        debug!("starting ui");
        TuiApplication::create(input_sx, should_quit, ui_state, context)?
            .render()
            .await
    }

    async fn handler_init(
        input_rx: Receiver<InputMessages>,
        app_sx: &Sender<AppEvents>,
        ui_state: &Arc<Mutex<UIContext<'static>>>,
        should_quit: &Arc<AtomicBool>,
    ) {
        debug!("starting input handler");
        tokio::spawn(InputHandler::init(
            input_rx,
            app_sx.clone(),
            ui_state.clone(),
            should_quit.clone(),
        ));
    }
}
