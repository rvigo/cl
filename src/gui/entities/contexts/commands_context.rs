use super::{namespaces_context::DEFAULT_NAMESPACE, Selectable};
use crate::{
    command::Command, commands::Commands, gui::entities::fuzzy::Fuzzy,
    resources::commands_file_service::CommandsFileService,
};
use anyhow::Result;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use itertools::Itertools;
use log::debug;
use std::{cmp::Reverse, collections::HashMap, thread, time::Duration};
use tui::widgets::ListState;

/// Caches a `Command` list using the namespace as a key for faster search
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
        self.cache
            .get(namespace)
            .unwrap_or(&Vec::default())
            .to_owned()
    }

    #[inline]
    pub fn update_entry(&mut self, new_command_item: &Command, old_command_item: &Command) {
        let new_namespace = &new_command_item.namespace;

        debug!("updating {new_namespace} cache entries with the new command");
        let commands = self.cache.entry(new_namespace.to_owned()).or_insert(vec![]);
        commands.push(new_command_item.to_owned());

        self.remove_entry(old_command_item);

        self.sort_cached_values()
    }

    #[inline]
    fn remove_entry(&mut self, command_item: &Command) {
        let namespace = &command_item.namespace;
        if let Some(commands) = self.cache.get_mut(namespace) {
            if let Some(index) = commands.iter().position(|c| c.eq(command_item)) {
                debug!("removing old cache entry from {namespace}");
                commands.remove(index);
            }
        }
    }

    #[inline]
    pub fn insert_entry(&mut self, command_item: Command) {
        let namespace = &command_item.namespace;
        if let Some(commands) = self.cache.get_mut(namespace) {
            commands.push(command_item)
        } else {
            self.cache.insert(namespace.to_string(), vec![command_item]);
        }

        self.sort_cached_values()
    }

    #[inline]
    fn sort_cached_values(&mut self) {
        for commands in self.cache.values_mut() {
            commands.sort_by_key(|c| c.alias.to_lowercase());
        }
    }
}

/// Groups all `Command`'s related stuff
pub struct CommandsContext {
    commands: Commands,
    state: ListState,
    to_be_executed: Option<Command>,
    commands_cache: CacheInfo,
    matcher: SkimMatcherV2,
    commands_file_service: CommandsFileService,
    filtered_commands: Vec<Command>,
}

impl CommandsContext {
    pub fn new(commands: Vec<Command>, commands_file_service: CommandsFileService) -> Self {
        let mut context = Self {
            commands: Commands::init(commands.clone()),
            state: ListState::default(),
            to_be_executed: None,
            commands_cache: CacheInfo::new(commands),
            matcher: SkimMatcherV2::default(),
            commands_file_service,
            filtered_commands: vec![],
        };
        context.state.select(Some(0));

        context
    }

    /// Returns a `ListState`, representing the state of the command list in the app
    pub fn state(&self) -> ListState {
        self.state.to_owned()
    }

    /// Selects the given command to be executed at the end of the app execution
    pub fn set_command_to_be_executed(&mut self, command: Option<Command>) {
        self.to_be_executed = command
    }

    pub fn get_selected_command_idx(&self) -> usize {
        self.state.selected().unwrap_or(0)
    }

    /// Resets the command index in the current command list
    pub fn reset_selected_command_idx(&mut self) {
        self.select_command_idx(0)
    }

    pub fn filter_commands(&mut self, current_namespace: &str, query_string: &str) {
        let commands = self.filter(current_namespace, query_string);

        if self.get_selected_command_idx() >= commands.len() {
            self.reset_selected_command_idx()
        }

        self.filtered_commands = commands;
    }

    pub fn filtered_commands(&self) -> Vec<Command> {
        self.filtered_commands.to_owned()
    }

    /// Adds a new command and then saves the updated `commands.toml` file
    pub fn add_command(&mut self, new_command: &Command) -> Result<()> {
        new_command.validate()?;
        debug!("new command validated: {new_command:?}");
        let commands = self.commands.add_command(new_command)?;

        self.commands_cache.insert_entry(new_command.to_owned());
        self.commands_file_service.save(&commands)?;
        Ok(())
    }

    /// Adds an edited command, deletes the older one and then saves the updated `commands.toml` file
    pub fn add_edited_command(
        &mut self,
        edited_command: &Command,
        current_command: &Command,
    ) -> Result<()> {
        edited_command.validate()?;
        let commands = self
            .commands
            .add_edited_command(edited_command, current_command)?;
        self.commands_cache
            .update_entry(edited_command, current_command);
        self.commands_file_service.save(&commands)?;
        Ok(())
    }

    /// Removes a command and then saves the updated `commands.toml` file
    pub fn remove_command(&mut self, command: &Command) -> Result<()> {
        let commands = self.commands.remove(command)?;
        self.commands_cache.remove_entry(command);
        self.commands_file_service.save(&commands)?;
        Ok(())
    }

    /// Runs a previously selected command
    ///
    /// If the command has any `named parameters`, will show a warning message
    pub fn execute_command(&self, quiet: bool) -> Result<()> {
        if let Some(command) = &self.command_to_be_executed() {
            debug!("executing selected command");
            if command.has_named_parameter() {
                eprintln!(
                    "Warning: This command appears to contain one or more named parameters placeholders. \
                    It may not run correctly using the interface.\n\
                If you want to use these parameters, please use the CLI (cl exec --help)"
                );

                let seconds_to_sleep = Duration::from_secs(3);
                thread::sleep(seconds_to_sleep);

                eprintln!();
            }

            self.commands.exec_command(command, false, quiet)?;
        }

        Ok(())
    }

    /// Filters the commands based on a query and a namespace
    ///
    /// First loads all namespaces from the `CacheInfo` (if available) and then filters them.   
    ///  
    /// If the chache is empty for the current namespace, searchs for all commands and then updates the cache
    ///
    /// ## Arguments
    /// * `current_namespace` - A &str representing the app current namespace
    /// * `query_string` - A &str representing the user's query string
    ///
    fn filter(&mut self, current_namespace: &str, query_string: &str) -> Vec<Command> {
        let commands = if !current_namespace.is_empty() && current_namespace != DEFAULT_NAMESPACE {
            self.commands_cache.get_entry(current_namespace)
        } else {
            let command_list = self.commands.command_list().to_owned();

            if command_list.is_empty() {
                vec![Command::default()]
            } else {
                command_list
            }
        };

        if !commands.is_empty() && !query_string.is_empty() {
            self.fuzzy_find(current_namespace, query_string, commands)
        } else {
            commands
        }
    }

    /// Does a fuzzy search in the given vec
    ///
    /// Tries to return an ordered vec based on the score
    #[inline(always)]
    fn fuzzy_find(
        &self,
        namespace: &str,
        query_string: &str,
        commands: Vec<Command>,
    ) -> Vec<Command> {
        if commands.is_empty() {
            return commands;
        }

        let mut scored_commands: Vec<(i64, Command)> = commands
            .iter()
            .cloned()
            .filter(|c| (namespace.eq(DEFAULT_NAMESPACE) || c.namespace.eq(namespace)))
            .filter_map(|c| {
                self.matcher
                    .fuzzy_indices(&c.lookup_string(), query_string)
                    .map(|(score, _)| (score, c))
                    .filter(|(score, _)| *score > 1)
            })
            .collect();

        scored_commands.sort_by_key(|&(score, _)| Reverse(score));
        scored_commands.into_iter().map(|(_, c)| c).collect_vec()
    }

    /// Selects the command index in the current command list
    fn select_command_idx(&mut self, idx: usize) {
        self.state.select(Some(idx))
    }

    fn command_to_be_executed(&self) -> Option<Command> {
        self.to_be_executed.to_owned()
    }
}

impl Selectable for CommandsContext {
    fn next(&mut self) {
        let i = self.get_selected_command_idx();
        let filtered_commands = &self.filtered_commands;
        let next = (i + 1) % filtered_commands.len();

        self.select_command_idx(next);
    }

    fn previous(&mut self) {
        let i = self.get_selected_command_idx();
        let filtered_commands = &self.filtered_commands;
        let previous = (i + filtered_commands.len() - 1) % filtered_commands.len();

        self.select_command_idx(previous);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::command::CommandBuilder;
    use std::env::temp_dir;

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
        CommandsContext::new(
            commands,
            CommandsFileService::new(temp_dir().join("commands.toml")),
        )
    }

    #[test]
    fn should_go_to_next() {
        let mut context = commands_context_builder(3);
        let current_namespace = DEFAULT_NAMESPACE;
        let query_string = "";
        context.filter_commands(current_namespace, query_string);

        assert_eq!(context.state.selected(), Some(0));

        context.next();
        assert_eq!(context.state().selected(), Some(1));

        context.next();
        assert_eq!(context.state().selected(), Some(2));

        context.next();
        assert_eq!(context.state().selected(), Some(0));
    }

    #[test]
    fn should_go_to_previous_command() {
        let mut context = commands_context_builder(3);
        let current_namespace = DEFAULT_NAMESPACE;
        let query_string = "";
        context.filter_commands(current_namespace, query_string);

        assert_eq!(context.state().selected(), Some(0));

        context.previous();
        assert_eq!(context.state().selected(), Some(2));

        context.previous();
        assert_eq!(context.state().selected(), Some(1));
    }

    #[test]
    fn should_add_a_command() {
        let mut context = commands_context_builder(3);

        // valid command
        let namespace = "new_namespace";
        let mut builder = CommandBuilder::default();
        builder
            .alias("new_command")
            .namespace(namespace)
            .command("command");
        let new_command = builder.build();

        let result = context.add_command(&new_command);
        assert!(result.is_ok());
        assert_eq!(context.commands_cache.get_entry(namespace).len(), 1);

        // invalid command
        let namespace = "invalid_namespace";
        let mut builder = CommandBuilder::default();
        builder.alias("new_command").namespace(namespace);
        let invalid_command = builder.build();

        let result = context.add_command(&invalid_command);
        assert!(result.is_err());
        assert!(context.commands_cache.cache.get(namespace).is_none())
    }

    #[test]
    fn should_remove_a_command() {
        let mut context = commands_context_builder(0);

        let mut builder = CommandBuilder::default();
        builder
            .alias("new_command")
            .namespace("namespace")
            .command("command");
        let command = builder.build();

        assert_eq!(context.commands.command_list().len(), 0);
        assert!(context.add_command(&command).is_ok());

        assert_eq!(context.commands.command_list().len(), 1);
        assert!(context.remove_command(&command).is_ok());

        assert_eq!(context.commands.command_list().len(), 0);
    }

    #[test]
    fn should_add_an_edited_command() {
        let mut context = commands_context_builder(1);

        let current_command_idx = context.get_selected_command_idx();
        let mut command_list = context.commands.command_list().to_owned();
        let current_command = command_list.get_mut(current_command_idx).unwrap();
        let mut edited_command = current_command.clone();
        edited_command.alias = "Edited_Alias".to_string();

        assert_eq!(context.commands.command_list().len(), 1);
        assert!(context
            .add_edited_command(&edited_command, current_command)
            .is_ok());
        assert_eq!(context.commands.command_list().len(), 1);
        assert!(context.commands.command_list().contains(&edited_command));
        assert!(!context.commands.command_list().contains(current_command))
    }

    #[test]
    fn should_filter_commands() {
        let mut context = commands_context_builder(4);
        context.filter_commands(DEFAULT_NAMESPACE, "");
        context.next();
        context.next();
        assert_eq!(context.get_selected_command_idx(), 2);

        context.filter_commands(DEFAULT_NAMESPACE, "4");

        assert_eq!(context.filtered_commands().len(), 1);
        let command = &context.filtered_commands()[0];

        assert_eq!(command.namespace, "namespace4");
        assert_eq!(context.get_selected_command_idx(), 0)
    }

    #[test]
    fn should_fuzzy_find_commands() {
        let command1 = Command {
            namespace: "git".to_owned(),
            command: "git log --oneline".to_owned(),
            description: None,
            alias: "gl".to_owned(),
            tags: None,
        };
        let command2 = Command {
            namespace: "git".to_owned(),
            command: "git fetch".to_owned(),
            description: None,
            alias: "gf".to_owned(),
            tags: None,
        };
        let command3 = Command {
            namespace: "cl".to_owned(),
            command: "cl --version".to_owned(),
            description: None,
            alias: "clv".to_owned(),
            tags: None,
        };
        let command4 = Command {
            namespace: "test".to_owned(),
            command: "command".to_owned(),
            description: Some("git mock command".to_owned()),
            alias: "some_string_with_c_and_l".to_owned(),
            tags: None,
        };

        let commands = vec![
            command1.clone(),
            command2.clone(),
            command3.clone(),
            command4.clone(),
        ];

        let mut context = CommandsContext::new(
            commands,
            CommandsFileService::new(temp_dir().join("commands.toml")),
        );

        context.filter_commands(DEFAULT_NAMESPACE, "git");

        assert_eq!(context.filtered_commands().len(), 3);
        assert!(&context.filtered_commands().contains(&command1));
        assert!(&context.filtered_commands().contains(&command2));
        assert!(&context.filtered_commands().contains(&command4));

        context.filter_commands(DEFAULT_NAMESPACE, "cl");
        assert_eq!(context.filtered_commands().len(), 2);
        assert!(&context.filtered_commands().contains(&command3));
        assert!(&context.filtered_commands().contains(&command4));
    }
}
