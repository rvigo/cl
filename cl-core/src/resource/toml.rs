use super::{errors::FileError, fs_wrapper::macros::read_to_string};
use crate::commands::CommandMap;
use anyhow::Result;
use std::path::Path;

pub struct TomlFileHandler;

impl TomlFileHandler {
    pub fn generate_commands_from_file<P>(&self, path: P) -> Result<CommandMap>
    where
        P: AsRef<Path>,
    {
        let string_data = read_to_string!(path)?;
        let commands = toml::from_str::<CommandMap>(&string_data)?;

        Ok(commands)
    }

    pub fn generate_file_from_commands(&self, commands: &CommandMap) -> Result<String, FileError> {
        toml::to_string(&commands).map_err(FileError::from)
    }
}
