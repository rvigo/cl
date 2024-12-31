use super::errors::FileError;
use crate::CommandMap;
use anyhow::Result;
use std::{fs::read_to_string, path::Path};

pub struct Toml;

impl Toml {
    pub fn from_file<'f, P>(path: P) -> Result<CommandMap<'f>>
    where
        P: AsRef<Path>,
    {
        let string_data = read_to_string(path)?;
        let commands = toml::from_str::<CommandMap>(&string_data)?;

        Ok(commands)
    }

    pub fn from_map(commands: &CommandMap) -> Result<String, FileError> {
        toml::to_string(&commands).map_err(FileError::from)
    }
}
