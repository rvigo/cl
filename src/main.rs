use anyhow::{Context, Result};
use cl_cli::{app::App, print_metadata, run_subcommands};
use cl_core::{
    config::Config,
    logger::{LoggerBuilder, LoggerType},
};
use cl_gui::start_gui;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().context("Cannot load the config file")?;

    let logger = LoggerBuilder::default()
        .with_log_level(config.preferences().log_level())
        .with_path(config.root_dir());

    let app = App::parse_app();

    if app.version {
        print_metadata()
    } else if let Some(subcommands) = app.subcommands {
        logger
            .with_logger_type(LoggerType::Subcommand)
            .build()
            .init()?;

        run_subcommands(subcommands, config)
    } else {
        logger
            .with_logger_type(LoggerType::MainApp)
            .build()
            .init()?;

        start_gui(config).await
    }
}
