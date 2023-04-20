mod entities;
mod key_handlers;
mod screens;

pub mod core {
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
        resources::{config::Config, file_service::FileService},
    };
    use anyhow::Result;
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
        let file_service = FileService::new(config.get_command_file_path()?);
        let commands = file_service.load_commands_from_file()?;

        debug!("creating contexts");
        let should_quit: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let ui_context = Arc::new(Mutex::new(UIContext::new()));
        let context = Arc::new(Mutex::new(ApplicationContext::init(
            commands,
            file_service,
            config.get_options(),
        )));

        debug!("creating terminal");
        let mut terminal = create_terminal()?;

        let size = terminal.size();

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
        .await?;

        Ok(())
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
    ) -> Result<()> {
        debug!("starting ui");
        TuiApplication::create(
            input_sx,
            should_quit,
            ui_context,
            context,
            terminal,
            screens,
        )?
        .render()
        .await
    }

    fn create_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
        Terminal::new()
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
