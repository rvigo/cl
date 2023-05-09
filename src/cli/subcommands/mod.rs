pub(super) mod config;
pub(super) mod exec;
pub(super) mod misc;
pub(super) mod share;

use crate::resources::config::Config;
use anyhow::Result;

/// Represents a CLI Subcommand
pub trait Subcommand {
    /// Runs the subcommand with the given `Config`
    fn run(&self, config: Config) -> Result<()>;
}

/// Loads a `Commands` instance from a command file at the given path
#[macro_export]
macro_rules! load_commands {
    ($command_file_path:expr) => {{
        use anyhow::{Context, Result};
        use std::path::PathBuf;
        use $crate::{commands::Commands, resources::file_service::FileService};

        fn load(command_file_path: PathBuf) -> Result<Commands> {
            let file_service = FileService::new(command_file_path).validate()?;
            let command_list = file_service
                .load()
                .context("Could not load the commands from file")?;
            let commands = Commands::init(command_list);
            Ok(commands)
        }
        load($command_file_path)
    }};
}

pub(super) use load_commands;
