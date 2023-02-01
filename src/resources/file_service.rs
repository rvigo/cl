use super::{config::CONFIG, utils::to_toml};
use crate::command::Command;
use anyhow::{bail, Context, Result};
use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    path::Path,
};

pub fn save_file(contents: String, path: &Path) -> Result<()> {
    if let Err(error) = write(path, contents) {
        bail!("Error writing to file {}: {}", path.display(), error)
    }
    Ok(())
}

pub fn open_file(path: &Path) -> Result<String> {
    let path_str = path.to_str().unwrap();
    match read_to_string(path_str) {
        Ok(file) => Ok(file),
        Err(error) => bail!("Cannot read {path_str}: {error}"),
    }
}

pub fn convert_from_toml_file(path: &Path) -> Result<Vec<Command>> {
    match toml::from_str::<HashMap<String, Vec<Command>>>(&open_file(path)?) {
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

pub fn load_commands_from_file() -> Result<Vec<Command>> {
    convert_from_toml_file(&CONFIG.get_command_file_path())
}

fn generate_toml(commands: &Vec<Command>) -> String {
    let mut map: HashMap<String, Vec<Command>> = HashMap::new();
    for command in commands {
        let item = command.to_owned();
        if let Some(commands) = map.get_mut(&item.namespace) {
            commands.push(item);
        } else {
            map.insert(item.clone().namespace, vec![item]);
        }
    }

    to_toml::<HashMap<String, Vec<Command>>>(&map)
}

pub fn write_toml_file(commands: &Vec<Command>, path: &Path) -> Result<()> {
    let toml = generate_toml(commands);
    save_file(toml, path)
}

pub fn write_to_command_file(commands: &Vec<Command>) -> Result<()> {
    let path = &CONFIG.get_command_file_path();
    write_toml_file(commands, path).context("Cannot write to the commands file")
}
