use anyhow::{Context, Result};
use cl_cli::{app::App, run_subcommands};
use cl_core::{
    logger::{LoggerBuilder, LoggerType},
    Config, DefaultConfig,
};
use cl_gui::start_gui;

#[tokio::main]
async fn main() -> Result<()> {
    let config = DefaultConfig::load().context("Cannot load the config file")?;

    let logger = LoggerBuilder::default()
        .with_log_level(config.preferences().log_level())
        .with_path(config.root_dir());

    let app = App::parse_app();

    if let Some(subcommands) = app.subcommands {
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
