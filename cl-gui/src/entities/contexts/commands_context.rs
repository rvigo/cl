use super::{
    namespaces::{Namespaces, DEFAULT_NAMESPACE},
    Selectable,
};
use crate::entities::{fuzzy::Fuzzy, states::State};
use anyhow::Result;
use cl_core::{
    command::Command, commands::Commands, resource::commands_file_handler::CommandsFileHandler,
    CommandVec, CommandVecExt, Namespace,
};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use itertools::Itertools;
use log::debug;
use std::{cmp::Reverse, thread, time::Duration};
use tui::widgets::ListState;

/// Groups all `Command`'s related stuff
pub struct CommandsContext {
    commands: Commands,
    state: ListState,
    pub namespaces: Namespaces,
    to_be_executed: Option<Command>,
    matcher: SkimMatcherV2,
    commands_file_handler: CommandsFileHandler,
    filtered_commands: CommandVec,
}

impl CommandsContext {
    pub fn new(commands: Commands, commands_file_handler: CommandsFileHandler) -> Self {
        let namespaces = commands
            .command_as_list()
            .iter()
            .map(|c| c.namespace.to_owned())
            .collect_vec();

        let mut context = Self {
            commands,
            namespaces: Namespaces::new(namespaces),
            state: ListState::default(), // TODO move this to another place
            to_be_executed: None,
            matcher: SkimMatcherV2::default(),
            commands_file_handler,
            filtered_commands: vec![],
        };

        context.state.select(Some(0));

        context
    }

    fn namespaces(&mut self) -> Vec<Namespace> {
        // let mut namespaces = self.namespaces.items.include_default();

        self.namespaces.items.to_owned()
    }

    pub fn current_namespace(&self) -> String {
        self.namespaces.items[self.namespaces.state.selected()].to_owned()
    }

    /// Returns a `ListState`, representing the state of the command list in the app
    pub fn state(&self) -> ListState {
        self.state.to_owned()
    }

    /// Selects the given command to be executed at the end of the app execution
    pub fn set_command_to_be_executed(&mut self, command: Option<Command>) {
        self.to_be_executed = command
    }

    pub fn selected_command_idx(&self) -> usize {
        self.state.selected().unwrap_or(0)
    }

    /// Resets the command index in the current command list
    pub fn reset_selected_command_idx(&mut self) {
        self.select_command_idx(0)
    }

    pub fn filter_commands(&mut self, current_namespace: &str, query_string: &str) -> CommandVec {
        let commands = self.filter(current_namespace, query_string);

        if self.selected_command_idx() >= commands.len() {
            self.reset_selected_command_idx()
        }

        self.filtered_commands = commands.to_owned();
        self.namespaces.items = self.namespaces();

        self.filtered_commands.to_owned()
    }

    pub fn filtered_commands(&self) -> CommandVec {
        self.filtered_commands.to_owned()
    }

    /// Adds a new command and then saves the updated `commands.toml` file
    pub fn add_command(&mut self, new_command: &Command) -> Result<()> {
        new_command.validate()?;

        debug!("new command validated: {new_command:?}");
        let commands = self.commands.add_command(new_command)?;

        self.namespaces
            .update_namespaces(&commands.keys().collect_vec());

        self.commands_file_handler.save(&commands)?;

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

        self.namespaces
            .update_namespaces(&commands.keys().collect_vec());

        self.commands_file_handler.save(&commands)?;

        Ok(())
    }

    /// Removes a command and then saves the updated `commands.toml` file
    pub fn remove_command(&mut self, command: &Command) -> Result<()> {
        let commands = self.commands.remove(command)?;

        // self.cache.remove_entry(command);
        self.commands_file_handler.save(&commands)?;

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
    fn filter(&mut self, current_namespace: &str, query_string: &str) -> CommandVec {
        let mut commands = {
            let commands =
                if !current_namespace.is_empty() && current_namespace != DEFAULT_NAMESPACE {
                    self.commands
                        .commands()
                        .get(current_namespace)
                        .unwrap_or(&vec![])
                        .to_owned()
                } else {
                    let command_list = self.commands.command_as_list();

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
        };

        commands.sort_and_return()
    }

    /// Does a fuzzy search in the given vec
    ///
    /// Tries to return an ordered vec based on the score
    #[inline(always)]
    fn fuzzy_find(&self, namespace: &str, query_string: &str, commands: CommandVec) -> CommandVec {
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
        let i = self.selected_command_idx();
        let filtered_commands = &self.filtered_commands;
        let next = (i + 1) % filtered_commands.len();

        self.select_command_idx(next);
    }

    fn previous(&mut self) {
        let i = self.selected_command_idx();
        let filtered_commands = &self.filtered_commands;
        let previous = (i + filtered_commands.len() - 1) % filtered_commands.len();

        self.select_command_idx(previous);
    }
}

impl Selectable for Namespaces {
    fn next(&mut self) {
        let current = self.state.selected();
        let next = (current + 1) % self.items.len();

        self.state.select(next);
    }

    fn previous(&mut self) {
        let current = self.state.selected();
        let previous = (current + self.items.len() - 1) % self.items.len();

        self.state.select(previous);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env::temp_dir;

    // macro_rules! create_command {
    //     ($alias:expr, $command:expr, $namespace:expr, $description:expr, $tags:expr) => {
    //         Command {
    //             alias: $alias.to_owned(),
    //             namespace: $namespace.to_owned(),
    //             command: $command.to_owned(),
    //             description: $description,
    //             tags: $tags,
    //         }
    //     };
    // }

    fn commands_builder(n_of_commands: usize) -> CommandVec {
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
            Commands::init(commands.to_command_map()),
            CommandsFileHandler::new(temp_dir().join("commands.toml")),
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
    fn should_filter_commands() {
        let mut context = commands_context_builder(4);
        context.filter_commands(DEFAULT_NAMESPACE, "");
        context.next();
        context.next();
        assert_eq!(context.selected_command_idx(), 2);

        context.filter_commands(DEFAULT_NAMESPACE, "4");

        assert_eq!(context.filtered_commands().len(), 1);
        let command = &context.filtered_commands()[0];

        assert_eq!(command.namespace, "namespace4");
        assert_eq!(context.selected_command_idx(), 0)
    }

    // #[test]
    // fn should_fuzzy_find_commands() {
    //     let command1 = create_command!("cl", "git log --oneline", "git", None, None);
    //     let command2 = create_command!("gf", "git fetch", "git", None, None);
    //     let command3 = create_command!("clv", "cl --version", "cl", None, None);
    //     let command4 = create_command!(
    //         "some_string_with_c_and_l",
    //         "command",
    //         "test",
    //         Some("git_mock_command".to_owned()),
    //         None
    //     );

    //     let commands = vec![
    //         command1.to_owned(),
    //         command2.to_owned(),
    //         command3.to_owned(),
    //         command4.to_owned(),
    //     ];

    //     let mut context = CommandsContext::new(
    //         Commands::init(commands.to_command_map()),
    //         CommandsFileHandler::new(temp_dir().join("commands.toml")),
    //     );

    //     context.filter_commands(DEFAULT_NAMESPACE, "git");

    //     assert_eq!(context.filtered_commands().len(), 3);
    //     assert!(&context.filtered_commands().contains(&command1));
    //     assert!(&context.filtered_commands().contains(&command2));
    //     assert!(&context.filtered_commands().contains(&command4));

    //     context.filter_commands(DEFAULT_NAMESPACE, "cl");
    //     assert_eq!(context.filtered_commands().len(), 2);
    //     assert!(&context.filtered_commands().contains(&command3));
    //     assert!(&context.filtered_commands().contains(&command4));
    // }

    // #[test]
    // fn should_filter_namespaces() {
    //     let expected = vec![DEFAULT_NAMESPACE.to_string(), "namespace1".to_string()];
    //     let context = Namespaces::new(vec!["namespace1".to_string()]);
    //     assert_eq!(context.items, expected);

    //     let namespaces = vec![
    //         "namespace1".to_string(),
    //         "namespace1".to_string(),
    //         "namespace1".to_string(),
    //     ];

    //     let expected = vec![DEFAULT_NAMESPACE.to_string(), "namespace1".to_string()];

    //     let context = Namespaces::new(namespaces);
    //     assert_eq!(context.items, expected);
    // }

    #[test]
    fn should_go_to_previous_namespace() {
        let mut context = commands_context_builder(2);

        assert_eq!(context.current_namespace(), DEFAULT_NAMESPACE);

        context.namespaces.previous();
        assert_eq!(context.current_namespace(), "namespace2");

        context.namespaces.previous();
        assert_eq!(context.current_namespace(), "namespace1");

        context.namespaces.previous();
        assert_eq!(context.current_namespace(), DEFAULT_NAMESPACE);
    }

    #[test]
    fn should_go_to_next_namespace() {
        let mut context = commands_context_builder(2);

        assert_eq!(context.current_namespace(), DEFAULT_NAMESPACE);

        context.namespaces.next();
        assert_eq!(context.current_namespace(), "namespace1");

        context.namespaces.next();
        assert_eq!(context.current_namespace(), "namespace2");

        context.namespaces.next();
        assert_eq!(context.current_namespace(), DEFAULT_NAMESPACE);
    }
}
