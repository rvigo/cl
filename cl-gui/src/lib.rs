pub mod entity;
pub mod key_handler;
pub mod screen;
pub mod widget;

use anyhow::Result;
use cl_core::config::Config;
use tui::style::Color;

pub const DEFAULT_TEXT_COLOR: Color = Color::Rgb(229, 229, 229);
pub const DEFAULT_SELECTED_COLOR: Color = Color::Rgb(201, 165, 249);

pub async fn start_gui(config: Config) -> Result<()> {
    core::init(config).await
}

mod core {
    use super::entity::terminal::Terminal;
    use crate::{
        entity::{
            context::{application_context::ApplicationContext, ui::UI},
            event::{app_event::AppEvent, input_event::InputMessages},
            event_handler::EventHandler,
            input_handler::InputHandler,
            tui_application::TuiApplication,
        },
        screen::Screens,
    };
    use anyhow::{Context, Result};
    use cl_core::{
        commands::Commands, config::Config, resource::commands_file_handler::CommandsFileHandler,
    };
    use log::debug;
    use parking_lot::Mutex;
    use std::sync::{atomic::AtomicBool, Arc};
    use tokio::sync::mpsc::{channel, Receiver, Sender};

    pub async fn init(config: Config) -> Result<()> {
        debug!("creating channels");
        let (app_sx, app_rx) = channel::<AppEvent>(16);
        let (input_sx, input_rx) = channel::<InputMessages>(16);

        debug!("loading commands from file");
        let file_service = CommandsFileHandler::new(config.command_file_path()).validate()?;
        let commands = file_service
            .load()
            .context("Cannot load commands from file")?;
        let commands = Commands::init(commands);

        debug!("creating terminal");
        let mut terminal = Terminal::new()?;

        let size = terminal.size();

        debug!("creating contexts");
        let should_quit: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let ui_context = Arc::new(Mutex::new(UI::new(size)));
        let context = Arc::new(Mutex::new(ApplicationContext::init(
            commands,
            file_service,
            config.preferences(),
        )));

        debug!("creating screens");
        let screens = Screens::default();

        debug!("starting components");
        start_input_handler(input_rx, &app_sx, &ui_context, &should_quit).await;
        start_event_handler(app_rx, &context, &ui_context, &should_quit).await;

        log::debug!("creating tui");
        let tui = TuiApplication::create(
            input_sx,
            should_quit,
            ui_context,
            context,
            terminal,
            screens,
        )?;

        start_ui(tui).await.and_then(|mut tui| tui.shutdown())
    }

    async fn start_event_handler(
        app_rx: Receiver<AppEvent>,
        context: &Arc<Mutex<ApplicationContext>>,
        ui_context: &Arc<Mutex<UI<'static>>>,
        should_quit: &Arc<AtomicBool>,
    ) {
        debug!("starting event listener");
        tokio::spawn(EventHandler::init(
            app_rx,
            context.to_owned(),
            ui_context.to_owned(),
            should_quit.to_owned(),
        ));
    }

    async fn start_ui(mut tui: TuiApplication<'_>) -> Result<TuiApplication<'_>> {
        debug!("starting ui");

        tui.render().await?;

        Ok(tui)
    }

    async fn start_input_handler(
        input_rx: Receiver<InputMessages>,
        app_sx: &Sender<AppEvent>,
        ui_context: &Arc<Mutex<UI<'static>>>,
        should_quit: &Arc<AtomicBool>,
    ) {
        debug!("starting input handler");
        tokio::spawn(InputHandler::init(
            input_rx,
            app_sx.to_owned(),
            ui_context.to_owned(),
            should_quit.to_owned(),
        ));
    }
}
