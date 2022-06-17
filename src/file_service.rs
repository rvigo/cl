use crate::{command_item::CommandItem, config, utils::to_toml};
use anyhow::Result;
use std::{collections::HashMap, fs};

fn load_config() -> config::Config {
    config::load_or_build_config().expect("cannot load config file")
}

fn open_file() -> String {
    let file_path = load_config().command_file_path.unwrap();
    match std::fs::read_to_string(file_path.to_str().unwrap()) {
        Ok(file) => file,
        Err(error) => panic!("cannot create a new commands.toml file: {error}"),
    }
}

pub fn load_commands_file<'a>() -> Vec<CommandItem> {
    match toml::from_str::<HashMap<String, Vec<CommandItem>>>(&open_file()) {
        Ok(toml) => {
            let mut items: Vec<CommandItem> = toml.into_iter().flat_map(|(_, c)| c).collect();
            items.sort();
            items
        }
        Err(error) => panic!("{error}"),
    }
}
pub fn write_to_file(values: Vec<CommandItem>) -> Result<()> {
    let mut map: HashMap<String, Vec<CommandItem>> = HashMap::new();
    for item in values {
        if let Some(commands) = map.get_mut(&item.clone().namespace) {
            commands.push(item);
        } else {
            map.insert(item.clone().namespace, vec![item]);
        }
    }

    let toml = to_toml::<HashMap<String, Vec<CommandItem>>>(&map);
    let file_path = load_config().command_file_path.unwrap();
    fs::write(file_path, toml).expect("Error writing the new command.");

    Ok(())
}
