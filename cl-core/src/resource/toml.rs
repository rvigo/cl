use super::{errors::FileError, fs_wrapper::read_to_string};
use crate::CommandMap;
use anyhow::Result;
use std::path::Path;

pub struct Toml;

impl Toml {
    pub fn from_file<P>(path: P) -> Result<CommandMap>
    where
        P: AsRef<Path>,
    {
        let string_data = read_to_string!(path)?;
        let commands = toml::from_str::<CommandMap>(&string_data)?;

        Ok(commands)
    }

    pub fn from_map(commands: &CommandMap) -> Result<String, FileError> {
        toml::to_string(&commands).map_err(FileError::from)
    }
}
