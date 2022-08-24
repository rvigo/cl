use super::{config::CONFIG, utils::to_toml};
use crate::command::Command;
use anyhow::{bail, Result};
use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    path::Path,
};

pub fn save_file(contents: String, path: &Path) -> Result<()> {
    if let Err(error) = write(path, contents) {
        bail!(
            "Error writing to file {}: {}",
            path.to_str().unwrap(),
            error
        )
    }
    Ok(())
}

pub fn open_file(path: &Path) -> Result<String> {
    let path_str = path.to_str().unwrap();
    match read_to_string(path_str) {
        Ok(file) => Ok(file),
        Err(error) => bail!("cannot read {}: {}", path_str, error),
    }
}

pub fn load_commands_from_file() -> Result<Vec<Command>> {
    match toml::from_str::<HashMap<String, Vec<Command>>>(&open_file(
        &CONFIG.get_command_file_path(),
    )?) {
        Ok(toml) => {
            let mut commands: Vec<Command> = toml
                .into_iter()
                .flat_map(|(_, _commands)| _commands)
                .collect();
            commands.sort();
            Ok(commands)
        }
        Err(error) => bail!("{error}"),
    }
}

pub fn write_to_command_file(commands: &Vec<Command>) -> Result<()> {
    let mut map: HashMap<String, Vec<Command>> = HashMap::new();
    for command in commands {
        let item = command.to_owned();
        if let Some(commands) = map.get_mut(&item.namespace) {
            commands.push(item);
        } else {
            map.insert(item.clone().namespace, vec![item]);
        }
    }

    let toml = to_toml::<HashMap<String, Vec<Command>>>(&map);
    let path = &CONFIG.get_command_file_path();
    if let Err(error) = save_file(toml, path) {
        bail!("Error writing the new command: {}", error)
    }
    Ok(())
}
