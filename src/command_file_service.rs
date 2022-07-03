use crate::{
    command_item::CommandItem,
    configs::file_config::{self},
    utils::to_toml,
};
use anyhow::Result;
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Clone, Default)]
pub struct CommandFileService {
    pub file_path: PathBuf,
}

impl CommandFileService {
    pub fn init() -> CommandFileService {
        let config = file_config::load().expect("cannot load config file");
        CommandFileService {
            file_path: config.command_file_path.unwrap(),
        }
    }

    fn open_file(&self) -> String {
        let file_path = &self.file_path;
        match std::fs::read_to_string(file_path.to_str().unwrap()) {
            Ok(file) => file,
            Err(error) => panic!("cannot create a new commands.toml file: {error}"),
        }
    }

    pub fn load_commands_from_file<'a>(&self) -> Vec<CommandItem> {
        match toml::from_str::<HashMap<String, Vec<CommandItem>>>(&self.open_file()) {
            Ok(toml) => {
                let mut items: Vec<CommandItem> = toml.into_iter().flat_map(|(_, c)| c).collect();
                items.sort();
                items
            }
            Err(error) => panic!("{error}"),
        }
    }

    pub fn write_to_file(&self, values: Vec<CommandItem>) -> Result<()> {
        let mut map: HashMap<String, Vec<CommandItem>> = HashMap::new();
        for item in values {
            if let Some(commands) = map.get_mut(&item.clone().namespace) {
                commands.push(item);
            } else {
                map.insert(item.clone().namespace, vec![item]);
            }
        }

        let toml = to_toml::<HashMap<String, Vec<CommandItem>>>(&map);
        let file_path = &self.file_path;
        fs::write(file_path, toml).expect("Error writing the new command.");

        Ok(())
    }
}
