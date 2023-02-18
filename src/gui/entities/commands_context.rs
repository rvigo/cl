use crate::{command::Command, commands::Commands, fuzzy::Fuzzy, resources::file_service};
use anyhow::{bail, Result};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use log::debug;
use std::{collections::HashMap, thread, time::Duration};
use tui::widgets::ListState;

#[derive(Default)]
struct CacheInfo {
    cache: HashMap<String, Vec<Command>>,
}

impl CacheInfo {
    pub fn new(command_list: Vec<Command>) -> CacheInfo {
        let mut namespace_map: HashMap<String, Vec<Command>> = HashMap::new();

        command_list.into_iter().for_each(|c| {
            namespace_map
                .entry(c.namespace.to_owned())
                .or_insert(Vec::new())
                .push(c);
        });
        let mut cache_info = CacheInfo {
            cache: namespace_map,
        };

        cache_info.sort_cached_values();
        cache_info
    }

    #[inline]
    pub fn get_entry(&mut self, namespace: &str) -> Vec<Command> {
        debug!("searching for {namespace} entries");
        let commands = self.cache.get(namespace).unwrap().to_owned();
        debug!("found {} commands in the {namespace} key", commands.len());
        commands
    }

    pub fn update_entry(&mut self, new_command_item: Command, old_command_item: Command) {
        let new_namespace = &new_command_item.namespace;

        if let Some(commands) = self.cache.get_mut(new_namespace) {
            debug!("updating {new_namespace} cache entries with the new command");
            commands.push(new_command_item);
        }
        self.remove_entry(old_command_item);

        self.sort_cached_values()
    }

    fn remove_entry(&mut self, command_item: Command) {
        let namespace = &command_item.namespace;
        if let Some(commands) = self.cache.get_mut(namespace) {
            if let Some(index) = commands.iter().position(|c| c.eq(&command_item)) {
                debug!("removing old cache entry from {namespace}");
                commands.remove(index);
            }
        }
    }

    pub fn insert_entry(&mut self, command_item: Command) {
        let namespace = &command_item.namespace;
        if let Some(commands) = self.cache.get_mut(namespace) {
            commands.push(command_item)
        } else {
            self.cache.insert(namespace.to_string(), vec![command_item]);
        }

        self.sort_cached_values()
    }

    fn sort_cached_values(&mut self) {
        for commands in self.cache.values_mut() {
            commands.sort_by_key(|c| c.alias.to_lowercase());
        }
    }
}

pub struct CommandsContext {
    commands: Commands,
    state: ListState,
    to_be_executed: Option<Command>,
    commands_cache: CacheInfo,
    matcher: SkimMatcherV2,
}

impl CommandsContext {
    pub fn new(commands: Vec<Command>) -> Self {
        let mut context = Self {
            commands: Commands::init(commands.clone()),
            state: ListState::default(),
            to_be_executed: None,
            commands_cache: CacheInfo::new(commands),
            matcher: SkimMatcherV2::default(),
        };
        context.state.select(Some(0));

        context
    }

    pub fn state(&self) -> ListState {
        self.state.to_owned()
    }

    pub fn command_to_be_executed(&self) -> Option<Command> {
        self.to_be_executed.to_owned()
    }

    pub fn set_command_to_be_executed(&mut self, command: Option<Command>) {
        self.to_be_executed = command
    }

    pub fn select_command(&mut self, idx: usize) {
        self.state.select(Some(idx))
    }

    pub fn get_selected_command_idx(&self) -> usize {
        self.state.selected().unwrap_or(0)
    }

    pub fn filter_commands(&mut self, current_namespace: &str, query_string: &str) -> Vec<Command> {
        let commands = if !current_namespace.is_empty() && current_namespace != "All" {
            debug!("getting cached entries for namespace `{current_namespace}`");
            self.commands_cache.get_entry(current_namespace)
        } else {
            debug!("loading all entries (namespace `{current_namespace}`, querystring: `{query_string}`)");
            let command_list = self.commands.command_list().to_owned();

            if command_list.is_empty() {
                vec![Command::default()]
            } else {
                command_list
            }
        };

        if commands.len() > 1 && !query_string.is_empty() {
            self.fuzzy_find(current_namespace, query_string, commands)
        } else {
            commands
        }
    }

    #[inline(always)]
    pub fn fuzzy_find(
        &self,
        namespace: &str,
        query_string: &str,
        commands: Vec<Command>,
    ) -> Vec<Command> {
        if commands.is_empty() {
            return commands;
        }

        commands
            .iter()
            .cloned()
            .filter(|c| {
                (namespace.eq("All") || c.namespace.eq(namespace))
                    && self
                        .matcher
                        .fuzzy_match(&c.lookup_string(), query_string)
                        .is_some()
            })
            .collect::<Vec<Command>>()
    }

    pub fn next_command(&mut self, current_namespace: &str, query_string: &str) {
        let mut i = self.get_selected_command_idx();
        let filtered_commands = self.filter_commands(current_namespace, query_string);
        if !filtered_commands.is_empty() {
            i = if i >= filtered_commands.len() - 1 {
                0
            } else {
                i + 1
            };
        }
        self.select_command(i);
    }

    pub fn previous_command(&mut self, current_namespace: &str, query_string: &str) {
        let mut i = self.get_selected_command_idx();
        let filtered_commands = self.filter_commands(current_namespace, query_string);
        if !filtered_commands.is_empty() {
            i = if i == 0 {
                filtered_commands.len() - 1
            } else {
                i - 1
            };
        };

        self.select_command(i);
    }

    pub fn add_command(&mut self, new_command: &Command) -> Result<()> {
        new_command.validate()?;
        if let Ok(commands) = self.commands.add_command(new_command) {
            self.commands_cache.insert_entry(new_command.to_owned());
            file_service::write_to_command_file(&commands)
        } else {
            bail!("Cannot save the new command")
        }
    }

    pub fn add_edited_command(
        &mut self,
        edited_command: &Command,
        current_command: &Command,
    ) -> Result<()> {
        edited_command.validate()?;
        if let Ok(commands) = self
            .commands
            .add_edited_command(edited_command, current_command)
        {
            self.commands_cache
                .update_entry(edited_command.to_owned(), current_command.to_owned());
            file_service::write_to_command_file(&commands)
        } else {
            bail!("Cannot save the edited command")
        }
    }

    pub fn remove_command(&mut self, command: &Command) -> Result<()> {
        if let Ok(commands) = self.commands.remove(command) {
            self.commands_cache.remove_entry(command.to_owned());
            file_service::write_to_command_file(&commands)
        } else {
            bail!("Cannot remove the command")
        }
    }

    pub fn execute_command(&self) -> Result<()> {
        if let Some(command) = &self.command_to_be_executed() {
            if command.has_named_parameter() {
                eprintln!(
                    "Warning: This command appears to contains one or more named parameters placeholders. \
                    It may not run correctly using the interface.\n\
                If you want to use these parameters, please use the CLI option (cl exec --help)"
                );

                let seconds_to_sleep = Duration::from_secs(3);
                thread::sleep(seconds_to_sleep);

                eprintln!();
            }

            self.commands.exec_command(command, false, false)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn commands_builder(n_of_commands: usize) -> Vec<Command> {
        let mut commands = vec![];
        for i in 0..n_of_commands {
            commands.push(Command {
                namespace: format!("namespace{}", (i + 1)),
                command: "command".to_string(),
                description: None,
                alias: "alias".to_string(),
                tags: None,
            })
        }

        commands
    }
    fn commands_context_builder(n_of_commands: usize) -> CommandsContext {
        let commands = commands_builder(n_of_commands);
        CommandsContext::new(commands)
    }
    #[test]
    fn should_go_to_next_command() {
        let mut context = commands_context_builder(3);
        let current_namespace = "All";
        let query_string = "";

        assert_eq!(context.state.selected(), Some(0));

        context.next_command(current_namespace, query_string);
        assert_eq!(context.state.selected(), Some(1));

        context.next_command(current_namespace, query_string);
        assert_eq!(context.state.selected(), Some(2));

        context.next_command(current_namespace, query_string);
        assert_eq!(context.state.selected(), Some(0));
    }
    #[test]
    fn should_go_to_previous_command() {
        let mut context = commands_context_builder(3);
        let current_namespace = "All";
        let query_string = "";

        assert_eq!(context.state.selected(), Some(0));

        context.previous_command(current_namespace, query_string);
        assert_eq!(context.state.selected(), Some(2));

        context.previous_command(current_namespace, query_string);
        assert_eq!(context.state.selected(), Some(1));
    }
}
