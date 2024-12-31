mod command;
mod command_builder;
mod commands;
mod preferences;
mod resource;

pub mod config;
pub mod logger;

pub use command::Command;
pub use command_builder::CommandBuilder;
pub use commands::CommandExec;
pub use commands::Commands;
pub use config::default_config::DefaultConfig;
pub use config::Config;
pub use config::LogLevel;
pub use preferences::Preferences;
pub use resource::errors::CommandError;
pub use resource::fs;

use std::collections::HashMap;

pub type Namespace = String;
pub type CommandVec<'cmd> = Vec<Command<'cmd>>;
pub type CommandMap<'cmd> = HashMap<Namespace, CommandVec<'cmd>>;

pub trait CommandVecExt<'cmd> {
    fn sort_and_return(&mut self) -> CommandVec<'cmd>;

    fn to_command_map(&self) -> CommandMap<'cmd>;

    fn filter(&self, predicate: impl Fn(&Command) -> bool) -> Vec<&Command<'cmd>>;
}

impl<'cmd> CommandVecExt<'cmd> for CommandVec<'cmd> {
    fn sort_and_return(&mut self) -> CommandVec<'cmd> {
        let mut sorted_commands = self.clone();
        sorted_commands.sort_by_key(|c| c.alias.to_lowercase());

        sorted_commands
    }

    fn to_command_map(&self) -> CommandMap<'cmd> {
        let mut command_map = CommandMap::new();

        for command in self {
            command_map
                .entry(command.namespace.to_string())
                .and_modify(|commands| commands.push(command.clone()))
                .or_insert_with(|| vec![command.clone()]);
        }
        command_map
    }

    fn filter(&self, predicate: impl Fn(&Command) -> bool) -> Vec<&Command<'cmd>> {
        self.iter().filter(|c| predicate(c)).collect()
    }
}

pub trait CommandMapExt<'cmd> {
    fn to_vec(&self) -> CommandVec<'cmd>;
}

impl<'cmd> CommandMapExt<'cmd> for CommandMap<'cmd> {
    fn to_vec(&self) -> CommandVec<'cmd> {
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

#[macro_export]
macro_rules! initialize_commands {
    ($command_file_path:expr) => {{
        use anyhow::Context;
        use $crate::fs;
        use $crate::Commands;

        let command_list =
            fs::load_from($command_file_path).context("Cannot load the command file")?;
        Commands::init(command_list)
    }};
}
