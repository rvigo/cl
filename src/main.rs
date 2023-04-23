mod cli;
mod command;
mod commands;
mod gui;
mod resources;

use anyhow::Result;
use clap::Parser;
use cli::{
    app::{App, Subcommands},
    subcommands::Subcommand,
};
use resources::{
    config::Config,
    logger::{self, ErrorInterceptor},
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    logger::init(config.get_log_level(), config.get_app_home_dir())?;

    let app = App::parse();

    match app.subcommands {
        Some(Subcommands::Exec(exec)) => exec.run(config),
        Some(Subcommands::Share(share)) => share.run(config),
        Some(Subcommands::Misc(misc)) => misc.run(config),
        Some(Subcommands::Config(subcommand_config)) => subcommand_config.run(config),
        _ => run_main_app(config).await,
    }
    .log_if_error()
}

async fn run_main_app(config: Config) -> Result<()> {
    gui::core::init(config).await
}
