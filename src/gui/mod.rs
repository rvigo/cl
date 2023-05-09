mod entities;
mod key_handlers;
mod screens;

use tui::style::Color;

pub const DEFAULT_TEXT_COLOR: Color = Color::Rgb(229, 229, 229);
pub const DEFAULT_SELECTED_COLOR: Color = Color::Rgb(201, 165, 249);

pub(super) mod core {
    use super::entities::terminal::Terminal;
    use crate::{
        gui::{
            entities::{
                contexts::{application_context::ApplicationContext, ui_context::UIContext},
                event_handler::EventHandler,
                events::{app_events::AppEvent, input_events::InputMessages},
                input_handler::InputHandler,
                tui_application::TuiApplication,
            },
            screens::Screens,
        },
        resources::{
            commands_file_service::CommandsFileService, config::Config,
            logger::interceptor::ErrorInterceptor,
        },
    };
    use anyhow::{Context, Result};
    use log::debug;
    use parking_lot::Mutex;
    use std::{
        io::Stdout,
        sync::{atomic::AtomicBool, Arc},
    };
    use tokio::sync::mpsc::{channel, Receiver, Sender};
    use tui::backend::CrosstermBackend;

    pub async fn init(config: Config) -> Result<()> {
        debug!("creating channels");
        let (app_sx, app_rx) = channel::<AppEvent>(16);
        let (input_sx, input_rx) = channel::<InputMessages>(16);

        debug!("loading commands from file");
        let file_service = CommandsFileService::new(config.get_command_file_path()).validate()?;
        let commands = file_service
            .load()
            .context("Cannot load commands from file")
            .log_error()?;

        debug!("creating terminal");
        let mut terminal = Terminal::new().log_error()?;

        let size = terminal.size();

        debug!("creating contexts");
        let should_quit: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let ui_context = Arc::new(Mutex::new(UIContext::new(size.clone().into())));
        let context = Arc::new(Mutex::new(ApplicationContext::init(
            commands,
            file_service,
            config.get_options(),
        )));

        debug!("creating screens with size {size:?}");
        let screens = Screens::new(size);

        debug!("starting components");
        start_input_handler(input_rx, &app_sx, &ui_context, &should_quit).await;
        start_event_handler(app_rx, &context, &ui_context, &should_quit).await;

        start_ui(
            input_sx,
            should_quit,
            ui_context,
            context,
            terminal,
            screens,
        )
        .await
        .log_error()
        .and_then(|mut tui| tui.shutdown())
    }

    async fn start_event_handler(
        app_rx: Receiver<AppEvent>,
        context: &Arc<Mutex<ApplicationContext>>,
        ui_context: &Arc<Mutex<UIContext<'static>>>,
        should_quit: &Arc<AtomicBool>,
    ) {
        debug!("starting event listener");
        tokio::spawn(EventHandler::init(
            app_rx,
            context.clone(),
            ui_context.clone(),
            should_quit.clone(),
        ));
    }

    async fn start_ui<'a>(
        input_sx: Sender<InputMessages>,
        should_quit: Arc<AtomicBool>,
        ui_context: Arc<Mutex<UIContext<'a>>>,
        context: Arc<Mutex<ApplicationContext>>,
        terminal: Terminal<CrosstermBackend<Stdout>>,
        screens: Screens<'a, CrosstermBackend<Stdout>>,
    ) -> Result<TuiApplication<'a>> {
        debug!("starting ui");
        let mut tui = TuiApplication::create(
            input_sx,
            should_quit,
            ui_context,
            context,
            terminal,
            screens,
        )?;

        tui.render().await?;

        Ok(tui)
    }

    async fn start_input_handler(
        input_rx: Receiver<InputMessages>,
        app_sx: &Sender<AppEvent>,
        ui_context: &Arc<Mutex<UIContext<'static>>>,
        should_quit: &Arc<AtomicBool>,
    ) {
        debug!("starting input handler");
        tokio::spawn(InputHandler::init(
            input_rx,
            app_sx.clone(),
            ui_context.clone(),
            should_quit.clone(),
        ));
    }
}
