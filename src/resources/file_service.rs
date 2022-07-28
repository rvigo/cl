use super::utils::to_toml;
use crate::command::Command;
use anyhow::{bail, Result};
use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    path::PathBuf,
};

#[derive(Default)]
pub struct CommandFileService {
    pub command_file_path: PathBuf,
}

impl CommandFileService {
    pub fn init(command_file_path: PathBuf) -> CommandFileService {
        CommandFileService { command_file_path }
    }

    fn open_command_file(&self) -> Result<String> {
        let file_path = &self.command_file_path;
        match read_to_string(file_path.to_str().unwrap()) {
            Ok(file) => Ok(file),
            Err(error) => bail!("cannot read commands.toml: {}", error),
        }
    }

    pub fn load_commands_from_file(&self) -> Result<Vec<Command>> {
        match toml::from_str::<HashMap<String, Vec<Command>>>(&self.open_command_file()?) {
            Ok(toml) => {
                let mut items: Vec<Command> = toml
                    .into_iter()
                    .flat_map(|(_, commands)| commands)
                    .collect();
                items.sort();
                Ok(items)
            }
            Err(error) => bail!("{error}"),
        }
    }

    pub fn write_to_command_file(&self, items: &Vec<Command>) -> Result<()> {
        let mut map: HashMap<String, Vec<Command>> = HashMap::new();
        for item in items {
            let item = item.to_owned();
            if let Some(commands) = map.get_mut(&item.namespace) {
                commands.push(item);
            } else {
                map.insert(item.clone().namespace, vec![item]);
            }
        }

        let toml = to_toml::<HashMap<String, Vec<Command>>>(&map);
        let file_path = &self.command_file_path;
        if let Err(error) = write(file_path, toml) {
            bail!("Error writing the new command: {}", error)
        }
        Ok(())
    }
}
