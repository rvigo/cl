use anyhow::{Context, Result};
use cl_cli::{
    app::{App, Subcommands},
    subcommands::Subcommand,
};
use cl_core::resources::{
    config::Config,
    logger::{self, LoggerType},
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().context("Cannot load the config file")?;
    let app = App::parse_app();

    if let Some(subcommands) = app.subcommands {
        run_subcommands(subcommands, config)
    } else {
        run_main_app(config).await
    }
}

fn run_subcommands(subcommands: Subcommands, config: Config) -> Result<()> {
    logger::init(
        config.get_log_level(),
        config.get_root_dir(),
        LoggerType::Subcommand,
    )
    .context("Cannot start the logger")?;

    match subcommands {
        Subcommands::Exec(exec) => exec.run(config),
        Subcommands::Share(share) => share.run(config),
        Subcommands::Misc(misc) => misc.run(config),
        Subcommands::Config(subcommand_config) => subcommand_config.run(config),
    }
}

async fn run_main_app(config: Config) -> Result<()> {
    logger::init(
        config.get_log_level(),
        config.get_root_dir(),
        LoggerType::MainApp,
    )
    .context("Cannot start the logger")?;

    cl_gui::core::init(config).await
}
