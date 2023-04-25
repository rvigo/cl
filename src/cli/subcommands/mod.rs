pub(super) mod config;
pub(super) mod exec;
pub(super) mod misc;
pub(super) mod share;

use std::path::PathBuf;

use crate::{
    commands::Commands,
    resources::{config::Config, file_service::FileService},
};
use anyhow::{Context, Result};

/// Represents a CLI Subcommand
pub trait Subcommand {
    /// Runs the subcommand with the given `Config`
    fn run(&self, config: Config) -> Result<()>;
}

// Subcommands aux method to load commands
pub(self) fn load_commands(command_file_path: PathBuf) -> Result<Commands> {
    let file_service = FileService::new(command_file_path);
    let command_list = file_service
        .load_commands_from_file()
        .context("Could not load the commands from file")?;
    let commands = Commands::init(command_list);

    Ok(commands)
}
