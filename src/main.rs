mod cli;
mod command;
mod commands;
mod gui;
mod resources;

use crate::gui::key_handlers::input_handler::InputHandler;
use anyhow::Result;
use clap::Parser;
use cli::{
    app::{App, SubCommand},
    subcommands::{
        config::config_subcommand, exec::exec_subcommand, misc::misc_subcommand,
        share::share_subcommand,
    },
};
use gui::{
    entities::{
        app_router::AppRouter,
        application_context::ApplicationContext,
        events::{app_events::AppEvents, input_events::InputMessages},
        tui_application::TuiApplication,
        ui_state::UiState,
    },
    layouts::TerminalSize,
};
use log::debug;
use parking_lot::Mutex;
use resources::{
    config::Config,
    file_service::FileService,
    logger::{self, ErrorInterceptor},
};
use std::sync::{atomic::AtomicBool, Arc};
use tokio::sync::mpsc::{Receiver, Sender};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    logger::init(config.get_log_level(), config.get_app_home_dir())?;

    let app = App::parse();

    match app.subcommand {
        Some(SubCommand::Exec(exec)) => exec_subcommand(exec, config),
        Some(SubCommand::Share(share)) => share_subcommand(share, config),
        Some(SubCommand::Misc(misc)) => misc_subcommand(misc, config),
        Some(SubCommand::Config(sub_command_config)) => {
            config_subcommand(sub_command_config, config)
        }
        _ => run_main_app(config).await,
    }
    .log_if_error()
}

async fn run_main_app(config: Config) -> Result<()> {
    debug!("creating channels");
    // TODO falta um cara pra ouvir os eventos do app (render, run e quit)
    let (app_sx, app_rx) = tokio::sync::mpsc::channel::<AppEvents>(32);
    let (input_sx, input_rx) = tokio::sync::mpsc::channel::<InputMessages>(32);

    debug!("loading commands");
    let file_service = FileService::new(config.get_command_file_path()?);
    let commands = file_service.load_commands_from_file()?;

    debug!("creating context");
    let should_quit: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    let context = ApplicationContext::init(
        commands,
        TerminalSize::Medium,
        file_service,
        config.get_options(),
        should_quit.clone(),
    );

    let ui_state = Arc::new(Mutex::new(UiState::new(context.terminal_size().clone())));
    let context = Arc::new(Mutex::new(context));
    debug!("starting components");
    handler_init(input_rx, &app_sx, &ui_state, &should_quit, &context).await;
    app_router_init(app_rx, &context, &ui_state, &should_quit).await;
    ui_init(input_sx, should_quit, ui_state, context).await?;

    Ok(())
}

async fn app_router_init(
    app_rx: Receiver<AppEvents>,
    context: &Arc<Mutex<ApplicationContext<'static>>>,
    ui_state: &Arc<Mutex<UiState>>,
    should_quit: &Arc<AtomicBool>,
) {
    tokio::spawn(AppRouter::init(
        app_rx,
        context.clone(),
        ui_state.clone(),
        should_quit.clone(),
    ));
}

async fn ui_init<'a>(
    input_sx: Sender<InputMessages>,
    should_quit: Arc<AtomicBool>,
    ui_state: Arc<Mutex<UiState>>,
    context: Arc<Mutex<ApplicationContext<'static>>>,
) -> Result<()> {
    debug!("starting ui");
    TuiApplication::create(input_sx, should_quit, ui_state, context)?
        .render()
        .await
}

async fn handler_init(
    input_rx: Receiver<InputMessages>,
    app_sx: &Sender<AppEvents>,
    ui_state: &Arc<Mutex<UiState>>,
    should_quit: &Arc<AtomicBool>,
    context: &Arc<Mutex<ApplicationContext<'static>>>,
) {
    debug!("starting input handler");
    tokio::spawn(InputHandler::init(
        input_rx,
        app_sx.clone(),
        ui_state.clone(),
        should_quit.clone(),
        context.clone(),
    ));
}
