use super::{errors::FileError, fs_wrapper::macros::read_to_string};
use crate::command::Command;
use anyhow::Result;
use std::{collections::HashMap, path::Path};

pub struct TomlFileHandler;

impl TomlFileHandler {
    pub fn generate_commands_from_file<P>(&self, path: P) -> Result<Vec<Command>>
    where
        P: AsRef<Path>,
    {
        let string_data = read_to_string!(path)?;
        let toml = toml::from_str::<HashMap<String, Vec<Command>>>(&string_data)?;
        let mut commands: Vec<Command> = toml
            .into_iter()
            .flat_map(|(_, _commands)| _commands)
            .collect();
        commands.sort();

        Ok(commands)
    }

    pub fn generate_file_from_commands(
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
