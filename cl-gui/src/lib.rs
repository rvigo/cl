mod clipboard;
mod context;
mod event;
mod fuzzy;
mod key_handler;
mod screen;
mod state;
mod terminal;
mod tui_application;
mod view_mode;
mod widget;

use clipboard::Clipboard;
use fuzzy::Fuzzy;
use state::State;
use terminal::Terminal;
use tui_application::TuiApplication;
use view_mode::ViewMode;

use anyhow::Result;
use cl_core::Config;
use tui::style::Color;

// TODO create a theme configuration
pub const DEFAULT_TEXT_COLOR: Color = Color::Rgb(205, 214, 244);
pub const DEFAULT_WIDGET_NAME_COLOR: Color = Color::Rgb(198, 208, 245);
pub const DEFAULT_SELECTED_COLOR: Color = Color::Rgb(186, 187, 241);
pub const DEFAULT_HIGH_LIGHT_COLOR: Color = Color::Rgb(249, 226, 175);
pub const DEFAULT_BACKGROUND_COLOR: Color = Color::Rgb(24, 24, 37);
pub const DEFAULT_INFO_COLOR: Color = Color::Rgb(166, 227, 161);

pub async fn start_gui(config: Config) -> Result<()> {
    core::init(config).await
}

mod core {
    use crate::{
        context::{Application, UI},
        event::{
            handler::{AppEventHandler, InputEventHandler},
            AppEvent, InputEvent,
        },
        screen::Screens,
        Terminal, TuiApplication,
    };
    use anyhow::{Context, Result};
    use cl_core::{resource::FileService, Commands, Config};
    use log::debug;
    use parking_lot::Mutex;
    use std::sync::{atomic::AtomicBool, Arc};
    use tokio::sync::mpsc::{channel, Receiver, Sender};

    pub async fn init(config: Config) -> Result<()> {
        debug!("creating channels");
        let (app_sx, app_rx) = channel::<AppEvent>(16);
        let (input_sx, input_rx) = channel::<InputEvent>(16);

        debug!("loading commands from file");
        let file_service = FileService::new(config.command_file_path()).validate()?;
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
        let context = Arc::new(Mutex::new(Application::init(
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
        context: &Arc<Mutex<Application>>,
        ui_context: &Arc<Mutex<UI<'static>>>,
        should_quit: &Arc<AtomicBool>,
    ) {
        debug!("starting event listener");
        tokio::spawn(AppEventHandler::init(
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
        input_rx: Receiver<InputEvent>,
        app_sx: &Sender<AppEvent>,
        ui_context: &Arc<Mutex<UI<'static>>>,
        should_quit: &Arc<AtomicBool>,
    ) {
        debug!("starting input handler");
        tokio::spawn(InputEventHandler::init(
            input_rx,
            app_sx.to_owned(),
            ui_context.to_owned(),
            should_quit.to_owned(),
        ));
    }
}
