use anyhow::{Context, Result};
use cl_cli::{app::App, run_subcommands};
use cl_core::{
    logger::{LoggerBuilder, LoggerType},
    Config, DefaultConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = DefaultConfig::load().context("Cannot load the config file")?;

    let logger = LoggerBuilder::default()
        .with_log_level(config.preferences().log_level())
        .with_path(config.log_dir_path()?);

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

        new_core::init(config).await
    }
}

mod new_core {
    use anyhow::Result;
    use cl_core::Config;
    use cl_gui::state::state_actor::StateActor;
    use cl_gui::ui::ui_actor::UiActor;
    use tokio::try_join;

    pub async fn init(config: impl Config + 'static) -> Result<()> {
        let (state_tx, state_rx) = tokio::sync::mpsc::channel(8);

        let mut state_actor = StateActor::new(config, state_rx)?;
        let mut ui_actor = UiActor::default();

        try_join!(state_actor.run(), ui_actor.run(state_tx))?;

        Ok(())
    }
}
