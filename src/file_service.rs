use crate::command_item::CommandItem;
use crate::config;
use std::collections::HashMap;

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

pub fn load_commands_file() -> Vec<CommandItem> {
    match toml::from_str::<HashMap<String, Vec<CommandItem>>>(&open_file()) {
        Ok(toml) => toml.into_iter().flat_map(|(_, c)| c).collect(),
        Err(error) => panic!("{error}"),
    }
}
