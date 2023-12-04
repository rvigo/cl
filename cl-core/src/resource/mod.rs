pub mod commands_file_handler;
pub mod errors;
pub mod fs_wrapper;
pub mod metadata;
pub mod toml;

/// Loads a `Commands` instance from a command file at the given path
#[macro_export]
macro_rules! load_commands {
    ($command_file_path:expr) => {{
        use anyhow::{Context, Result};
        use std::path::PathBuf;
        use $crate::{commands::Commands, resource::commands_file_handler::CommandsFileHandler};

        let file_service = CommandsFileHandler::new($command_file_path).validate()?;
        let command_list = file_service
            .load()
            .context("Could not load the commands from file")?;
        let commands = Commands::init(command_list);

        Ok::<Commands, anyhow::Error>(commands)
    }};
}
