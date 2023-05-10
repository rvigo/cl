pub(super) mod commands_file_service;
pub(super) mod config;
pub(super) mod errors;
pub(super) mod logger;

/// Loads a `Commands` instance from a command file at the given path
#[macro_export]
macro_rules! load_commands {
    ($command_file_path:expr) => {{
        use anyhow::{Context, Result};
        use std::path::PathBuf;
        use $crate::{commands::Commands, resources::commands_file_service::CommandsFileService};

        fn load(command_file_path: PathBuf) -> Result<Commands> {
            let file_service = CommandsFileService::new(command_file_path).validate()?;
            let command_list = file_service
                .load()
                .context("Could not load the commands from file")?;
            let commands = Commands::init(command_list);
            Ok(commands)
        }
        load($command_file_path)
    }};
}

pub use load_commands;

/// Custom FileSystem Wrapper
pub(super) mod fs_wrapper {
    use super::errors::FileError;
    use anyhow::Result;
    use std::{fs, path::Path};

    pub fn write<P, C>(path: P, contents: C) -> Result<(), FileError>
    where
        P: AsRef<Path>,
        C: AsRef<[u8]>,
    {
        fs::write(&path, contents).map_err(|cause| FileError::WriteFile {
            path: path.as_ref().to_path_buf(),
            cause: cause.into(),
        })
    }

    pub fn read_to_string<P>(path: P) -> Result<String, FileError>
    where
        P: AsRef<Path>,
    {
        fs::read_to_string(&path).map_err(|cause| FileError::ReadFile {
            path: path.as_ref().to_path_buf(),
            cause: cause.into(),
        })
    }

    pub fn create_dir_all<P>(path: P) -> Result<(), FileError>
    where
        P: AsRef<Path>,
    {
        std::fs::create_dir_all(&path).map_err(|cause| FileError::CreateDirs {
            path: path.as_ref().to_path_buf(),
            cause,
        })
    }
}

pub(super) mod toml {
    use super::{errors::FileError, fs_wrapper::read_to_string};
    use crate::command::Command;
    use anyhow::Result;
    use std::{collections::HashMap, path::Path};

    pub trait TomlFileHandler {
        fn generate_commands_from_toml<P>(&self, path: P) -> Result<Vec<Command>>
        where
            P: AsRef<Path>,
        {
            let string_data = read_to_string(path)?;
            let toml = toml::from_str::<HashMap<String, Vec<Command>>>(&string_data)?;
            let mut commands: Vec<Command> = toml
                .into_iter()
                .flat_map(|(_, _commands)| _commands)
                .collect();
            commands.sort();

            Ok(commands)
        }

        fn generate_toml_from_commands(
            &self,
            commands: &Vec<Command>,
        ) -> Result<String, FileError> {
            let mut map: HashMap<String, Vec<Command>> = HashMap::new();
            for command in commands {
                let item = command.to_owned();
                if let Some(commands) = map.get_mut(&item.namespace) {
                    commands.push(item);
                } else {
                    map.insert(item.namespace.to_owned(), vec![item]);
                }
            }

            toml::to_string(&map).map_err(FileError::from)
        }
    }
}
