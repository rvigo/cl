pub mod errors;
mod file_service;
pub mod fs_wrapper;
mod toml;

pub use file_service::FileService;

/// Loads a `Commands` instance from a command file at the given path
#[macro_export]
macro_rules! load_commands {
    ($command_file_path:expr) => {{
        use anyhow::{Context, Result};
        use std::path::PathBuf;
        use $crate::{resource::FileService, Commands};

        let file_service = FileService::new($command_file_path)?;
        let command_list = file_service
            .load()
            .context("Could not load the commands from file")?;
        let commands = Commands::init(command_list);

        Ok::<Commands, anyhow::Error>(commands)
    }};
}
