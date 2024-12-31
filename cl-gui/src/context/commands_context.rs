use super::{
    namespace_context::{NamespaceContext, DEFAULT_NAMESPACE},
    Selectable,
};
use crate::{state::ListState, Fuzzy, State};
use anyhow::Result;
use cl_core::{fs, Command, CommandExec, CommandMap, CommandVec, CommandVecExt, Commands};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use itertools::Itertools;
use log::debug;
use std::{borrow::Cow, cmp::Reverse, path::PathBuf, thread, time::Duration};

/// Groups all `Command`'s related stuff
pub struct CommandsContext<'ctx> {
    command_file_path: PathBuf,
    commands: Commands<'ctx>,
    filtered_commands: CommandVec<'ctx>,
    matcher: SkimMatcherV2,
    state: ListState,
    to_be_executed: Option<Command<'ctx>>,
}

impl<'ctx> CommandsContext<'ctx> {
    pub fn new(commands: Commands<'ctx>, command_file_path: PathBuf) -> Self {
        let mut context = Self {
            command_file_path,
            commands,
            filtered_commands: vec![],
            matcher: SkimMatcherV2::default(),
            state: ListState::default(), // TODO move this to another place
            to_be_executed: None,
        };

        context.state.select(Some(0));

        context
    }
}

// Command manipulation
impl<'ctx> CommandsContext<'ctx> {
    /// Adds a new command and then saves the updated `commands.toml` file
    pub fn add(&mut self, new_command: &Command<'ctx>) -> Result<&CommandMap<'ctx>> {
        new_command.validate()?;

        self.commands.add(new_command).and_then(|cmds| {
            fs::save_at(cmds, &self.command_file_path)?;
            Ok(cmds)
        })
    }

    /// Adds an edited command, deletes the older one and then saves the updated `commands.toml` file
    pub fn edit(
        &mut self,
        edited_command: &Command<'ctx>,
        current_command: &Command<'ctx>,
    ) -> Result<&CommandMap<'ctx>> {
        edited_command.validate()?;

        self.commands
            .edit(edited_command, current_command)
            .and_then(|cmds| {
                fs::save_at(cmds, &self.command_file_path)?;
                Ok(cmds)
            })
    }

    /// Removes a command and then saves the updated `commands.toml` file
    pub fn remove(&mut self, command: &Command<'ctx>) -> Result<&CommandMap<'ctx>> {
        self.commands.remove(command).and_then(|cmds| {
            fs::save_at(cmds, &self.command_file_path)?;
            Ok(cmds)
        })
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
    fn filter(&mut self, current_namespace: &str, query_string: &str) -> CommandVec<'ctx> {
        let commands = if current_namespace != DEFAULT_NAMESPACE {
            let command_namespace = self.commands.get(current_namespace);

            let result = if let Some(command_namespace) = command_namespace {
                Cow::Borrowed(command_namespace)
            } else {
                let command_list = self.commands.as_list();
                Cow::Owned(command_list)
            };

            result
        } else {
            let command_list = self.commands.as_list();

            if command_list.is_empty() {
                Cow::Owned(vec![Command::default()])
            } else {
                Cow::Owned(command_list)
            }
        };

        let commands = if !commands.is_empty() && !query_string.is_empty() {
            let fuzzy_commands =
                self.fuzzy_find(current_namespace, query_string, commands.to_vec());
            Cow::Owned(fuzzy_commands)
        } else {
            commands
        };

        commands.to_vec().sort_and_return()
    }

    /// Does a fuzzy search in the given vec
    ///
    /// Tries to return an ordered vec based on the score
    #[inline(always)]
    fn fuzzy_find(
        &self,
        namespace: &str,
        query_string: &str,
        commands: CommandVec<'ctx>,
    ) -> CommandVec<'ctx> {
        if commands.is_empty() {
            return commands;
        }

        let mut scored_commands: Vec<(i64, Command)> = commands
            .iter()
            .filter(|&c| (namespace.eq(DEFAULT_NAMESPACE) || c.namespace.eq(namespace)))
            .cloned()
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
}

impl<'ctx> CommandsContext<'ctx> {
    /// Returns a `ListState`, representing the state of the command list in the app
    pub fn state(&self) -> ListState {
        self.state.to_owned()
    }

    /// Selects the given command to be executed at the end of the app execution
    pub fn set_command_to_be_executed(&mut self, command: Command<'ctx>) {
        self.to_be_executed = Some(command)
    }

    pub fn selected_command_idx(&self) -> usize {
        self.state.selected().unwrap_or(0)
    }

    /// Resets the command index in the current command list
    pub fn reset_selected_command_idx(&mut self) {
        self.select_command_idx(0)
    }

    pub fn filter_commands(
        &mut self,
        current_namespace: &str,
        query_string: &str,
    ) -> CommandVec<'ctx> {
        let idx = self.selected_command_idx();
        let commands = {
            let filtered = self.filter(current_namespace, query_string);
            filtered
        };

        if idx >= commands.len() {
            self.state.select(Some(0))
        }

        self.filtered_commands = commands.to_vec();

        self.filtered_commands.to_owned()
    }

    /// Runs a previously selected command
    ///
    /// If the command has any `named parameters`, will show a warning message
    pub fn execute(&self, quiet: bool) -> Result<()> {
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

            command.exec(false, quiet)?;
        }

        Ok(())
    }

    /// Selects the command index in the current command list
    fn select_command_idx(&mut self, idx: usize) {
        self.state.select(Some(idx))
    }

    fn command_to_be_executed(&self) -> Option<Command<'ctx>> {
        self.to_be_executed.to_owned()
    }
}

impl Selectable for CommandsContext<'_> {
    fn next(&mut self) {
        if self.filtered_commands.is_empty() {
            self.state.select(None);
            return;
        }

        let current = self.selected_command_idx();
        let next = (current + 1) % self.filtered_commands.len();

        self.state.select(Some(next))
    }

    fn previous(&mut self) {
        if self.filtered_commands.is_empty() {
            self.state.select(None);
            return;
        }

        let current = self.selected_command_idx();
        let filtered_commands = &self.filtered_commands;
        let previous = (current + filtered_commands.len() - 1) % filtered_commands.len();

        self.state.select(Some(previous))
    }
}

impl Selectable for NamespaceContext {
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
    use std::borrow::Cow;
    use std::vec;
    use tempdir::TempDir;

    macro_rules! create_command {
        ($alias:expr, $command:expr, $namespace:expr, $description:expr, $tags:expr) => {
            Command {
                alias: Cow::Borrowed($alias),
                namespace: Cow::Borrowed($namespace),
                command: Cow::Borrowed($command),
                description: $description,
                tags: $tags,
            }
        };
    }

    macro_rules! commands_context {
        ($commands:expr, $path:expr) => {
            CommandsContext::new(
                Commands::init($commands.to_command_map()),
                $path.join("commands.toml"),
            )
        };
    }

    #[test]
    fn should_save_a_command() {
        let tmp = TempDir::new("commands").unwrap();
        let command = create_command!("alias", "command", "namespace", None, None);
        let mut context = commands_context!(vec![command.to_owned()], tmp.path());
        let another_command = create_command!("alias2", "command", "namespace", None, None);

        let result = context.add(&another_command);

        assert!(result.is_ok());
        let commands_lock = context.commands;
        let commands = commands_lock.get(&command.namespace).unwrap();

        assert_eq!(commands.len(), 2);
    }

    #[test]
    fn should_remove_a_command() {
        let tmp = TempDir::new("commands").unwrap();
        let command = create_command!("alias", "command", "namespace", None, None);
        let mut context = commands_context!(vec![command.to_owned()], tmp.path());

        let result = context.remove(&command);

        assert!(result.is_ok());
        let commands_lock = context.commands;
        let entry = commands_lock.get(&command.namespace);

        assert!(entry.is_none());
    }

    #[test]
    fn should_go_to_next() {
        let tmp = TempDir::new("commands").unwrap();
        let command1 = create_command!("alias1", "command", "namespace", None, None);
        let command2 = create_command!("alias2", "command", "namespace", None, None);
        let command3 = create_command!("alias3", "command", "namespace", None, None);
        let mut context = commands_context!(vec![command1, command2, command3], tmp.path());
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
        let tmp = TempDir::new("commands").unwrap();
        let command1 = create_command!("alias1", "command", "namespace", None, None);
        let command2 = create_command!("alias2", "command", "namespace", None, None);
        let command3 = create_command!("alias3", "command", "namespace", None, None);
        let mut context = commands_context!(vec![command1, command2, command3], tmp.path());
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
        let tmp = TempDir::new("commands").unwrap();
        let command1 = create_command!("alias1", "command", "namespace", None, None);
        let command2 = create_command!("alias2", "command", "namespace", None, None);
        let command3 = create_command!("alias3", "command", "namespace", None, None);
        let command4 = create_command!("alias4", "command", "namespace", None, None);
        let mut context = commands_context!(
            vec![command1, command2, command3, command4.to_owned()],
            tmp.path()
        );
        context.filter_commands(DEFAULT_NAMESPACE, "");
        context.next();
        context.next();
        assert_eq!(context.selected_command_idx(), 2);

        context.filter_commands(DEFAULT_NAMESPACE, "4");

        assert_eq!(context.filtered_commands.len(), 1);
        let command = &context.filtered_commands[0];

        assert_eq!(command, &command4);
        assert_eq!(context.selected_command_idx(), 0)
    }

    #[test]
    fn should_fuzzy_find_commands() {
        // let command1 = create_command!("cl", "git log --oneline", "git", None, None);
        let command2 = create_command!("gf", "git fetch", "git", None, None);
        let command3 = create_command!("clv", "cl --version", "cl", None, None);
        // let command4 = create_command!(
        //     "some_string_with_c_and_l",
        //     "command",
        //     "test",
        //     Some("git_mock_command".to_owned()),
        //     None
        // );

        let commands = vec![
            // command1.to_owned(),
            command2.to_owned(),
            command3.to_owned(),
            // command4.to_owned(),
        ];

        let tmp = TempDir::new("commands").unwrap();
        let mut context = CommandsContext::new(
            Commands::init(commands.to_command_map()),
            tmp.path().join("commands.toml"),
        );

        context.filter_commands(DEFAULT_NAMESPACE, "git");
        let filtered = context.filtered_commands;
        println!("asserting git");
        assert_eq!(filtered.len(), 1);
        // assert!(&filtered.contains(&command1));
        assert!(&filtered.contains(&command2));
        // assert!(&filtered.contains(&command4));

        // println!("asserting cl");
        // context.filter_commands(DEFAULT_NAMESPACE, "cl");
        // assert_eq!(filtered.len(), 1);
        // assert!(&context.filtered_commands().contains(&command1));
        // assert!(filtered.contains(&command3));
        // assert!(&context.filtered_commands().contains(&command4));
    }
}
