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

use itertools::Itertools;
use std::collections::HashMap;

pub type Namespace = String;
pub type CommandVec<'cmd> = Vec<Command<'cmd>>;
pub type CommandMap<'cmd> = HashMap<Namespace, CommandVec<'cmd>>;

pub trait CommandVecExt<'cmd> {
    fn sorted(&mut self) -> CommandVec<'cmd>;

    fn to_command_map(&self) -> CommandMap<'cmd>;

    fn filter(&self, predicate: impl Fn(&Command) -> bool) -> Vec<&Command<'cmd>>;

    fn get_selected(&self, idx: usize) -> Command<'cmd>;

    fn first(&self) -> Option<Command<'cmd>>;

    fn aliases(&self) -> Vec<String>;

    fn namespaces(&self) -> Vec<String>;

    fn as_map(&self) -> CommandMap<'cmd>;
}

impl<'cmd> CommandVecExt<'cmd> for CommandVec<'cmd> {
    fn sorted(&mut self) -> CommandVec<'cmd> {
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

    fn get_selected(&self, idx: usize) -> Command<'cmd> {
        self.get(idx)
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| CommandBuilder::default().build())
            .to_owned()
    }

    #[allow(clippy::get_first)]
    fn first(&self) -> Option<Command<'cmd>> {
        self.get(0).cloned()
    }

    fn aliases(&self) -> Vec<String> {
        self.iter().map(|cmd| cmd.alias.to_string()).collect()
    }

    fn namespaces(&self) -> Vec<String> {
        self.iter().map(|cmd| cmd.namespace.to_string()).collect()
    }

    fn as_map(&self) -> CommandMap<'cmd> {
        self.iter()
            .chunk_by(|c| &c.namespace)
            .into_iter()
            .map(|(n, c)| (n.to_string(), c.cloned().collect()))
            .collect::<CommandMap<'cmd>>()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CommandBuilder;

    fn make_cmd(alias: &'static str, namespace: &'static str) -> Command<'static> {
        CommandBuilder::default()
            .alias(alias)
            .namespace(namespace)
            .command("echo test")
            .build()
    }

    #[test]
    fn first_returns_none_on_empty_vec() {
        let empty: CommandVec<'static> = vec![];
        assert!(empty.first().is_none());
    }

    #[test]
    fn first_returns_some_on_non_empty_vec() {
        let cmd = make_cmd("alias", "ns");
        let vec: CommandVec<'static> = vec![cmd.clone()];
        assert_eq!(vec.first(), Some(cmd));
    }
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
