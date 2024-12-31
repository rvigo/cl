pub mod context;

mod clipboard;
mod event;
mod fuzzy;
mod key_handler;
mod screen;
mod state;
mod terminal;
mod theme;
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

pub async fn start_gui(config: impl Config) -> Result<()> {
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
    use anyhow::Result;
    use cl_core::{initialize_commands, Config};
    use log::debug;
    use parking_lot::Mutex;
    use std::sync::{atomic::AtomicBool, Arc};
    use tokio::sync::mpsc::{channel, Receiver, Sender};

    pub async fn init(config: impl Config) -> Result<()> {
        debug!("creating channels");
        let (app_sx, app_rx) = channel::<AppEvent>(16);
        let (input_sx, input_rx) = channel::<InputEvent>(16);

        debug!("loading commands from file");
        let commands = initialize_commands!(config.command_file_path());

        debug!("creating terminal");
        let mut terminal = Terminal::new()?;

        let size = terminal.size();

        debug!("creating contexts");
        let should_quit: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let ui_context = Arc::new(Mutex::new(UI::new(size)));
        let context = Arc::new(Mutex::new(Application::init(
            commands,
            config.command_file_path(),
            config.preferences().to_owned(),
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
        context: &Arc<Mutex<Application<'static>>>,
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
