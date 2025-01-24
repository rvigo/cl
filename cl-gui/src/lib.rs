mod context;

mod clipboard;
mod event;
mod fuzzy;
mod key_handler;
mod screen;
mod state;
mod sync_cell;
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
    // core::init(config).await

    new_core::init().await

}

mod new_core {
    use anyhow::Result;
    use cl_new_gui::state::state_actor::StateActor;
    use cl_new_gui::ui::ui_actor::UiActor;
    use tokio::try_join;

    pub async fn init() -> Result<()> {
        let (state_tx, state_rx) = tokio::sync::mpsc::channel(8);

        let mut state_actor = StateActor::new(state_rx);
        let mut ui_actor = UiActor::new();

        try_join!(state_actor.run(), ui_actor.run(state_tx))?;

        Ok(())
    }
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
    use tokio::{select, try_join};

    pub async fn init(config: impl Config) -> Result<()> {
        debug!("starting events");
        // starting events
        InputEvent::init();
        AppEvent::init();

        let app_rx = AppEvent::get();
        let input_rx = InputEvent::get();

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
        let input_handler = start_input_handler(&ui_context, &should_quit);
        let event_handler = start_event_handler(&context, &ui_context, &should_quit);
        debug!("creating tui");
        let mut tui = TuiApplication::create(should_quit, ui_context, context, terminal, screens)?;

        // tokio::spawn(input_handler.handle(input_rx));
        // tokio::spawn(event_handler.handle(app_rx));
        select! {
            _ = input_handler.handle(input_rx) => {debug!("input handler done")},
            _ = event_handler.handle(app_rx) => {debug!("event handler done")},
            _ = tui.render() => {debug!("tui done")},
        }
        // input_handler.handle(input_rx), event_handler.handle(app_rx))?;
        // tui.render().await?;
        tui.shutdown()
    }

    fn start_event_handler(
        context: &Arc<Mutex<Application<'static>>>,
        ui_context: &Arc<Mutex<UI<'static>>>,
        should_quit: &Arc<AtomicBool>,
    ) -> AppEventHandler<'static> {
        debug!("starting event listener");
        AppEventHandler::init(
            context.to_owned(),
            ui_context.to_owned(),
            should_quit.to_owned(),
        )
    }

    fn start_input_handler(
        ui_context: &Arc<Mutex<UI<'static>>>,
        should_quit: &Arc<AtomicBool>,
    ) -> InputEventHandler {
        debug!("starting input handler");
        InputEventHandler::init(ui_context.to_owned(), should_quit.to_owned())
    }
}
