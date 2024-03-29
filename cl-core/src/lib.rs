pub mod cache;
pub mod command;
pub mod commands;
pub mod config;
pub mod logger;
pub mod preferences;
pub mod resource;

use command::Command;
use std::collections::HashMap;

pub type Namespace = String;
pub type CommandVec = Vec<Command>;
pub type CommandMap = HashMap<Namespace, CommandVec>;

pub trait CommandVecExt {
    fn sort_and_return(&mut self) -> CommandVec;

    fn to_command_map(&self) -> CommandMap;
}

impl CommandVecExt for CommandVec {
    fn sort_and_return(&mut self) -> CommandVec {
        self.sort_by_key(|c| c.alias.to_lowercase());
        self.iter_mut()
            .map(|c| c.to_owned())
            .collect::<CommandVec>()
    }

    fn to_command_map(&self) -> CommandMap {
        let mut command_map = CommandMap::new();
        for command in self {
            command_map
                .entry(command.namespace.to_owned())
                .or_insert_with(|| vec![command.to_owned()]);
        }
        command_map
    }
}

pub trait CommandMapExt {
    fn to_vec(&self) -> CommandVec;
}

impl CommandMapExt for CommandMap {
    fn to_vec(&self) -> CommandVec {
        self.iter()
            .flat_map(|(_, commands)| commands)
            .cloned()
            .collect()
    }
}

#[macro_export]
macro_rules! hashmap {
        () => {{
            std::collections::HashMap::new()
        }};

        ($($key:expr => $value:expr),* ) => {{
            let mut map = std::collections::HashMap::new();
            $(
            map.insert($key, $value);
            ) +

            map
        }};
    }
