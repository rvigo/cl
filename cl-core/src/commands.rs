use crate::Command;
use crate::CommandError;
use crate::CommandMap;
use crate::CommandMapExt;
use crate::CommandVec;
use crate::CommandVecExt;

use anyhow::{bail, Context, Result};
use std::{borrow::Borrow, env};
use tracing::{debug, trace, warn};

#[derive(Default)]
pub struct Commands<'cmd> {
    commands: CommandMap<'cmd>,
}

impl<'cmd> Commands<'cmd> {
    pub fn init(commands: CommandMap<'cmd>) -> Self {
        Commands { commands }
    }

    pub fn add(&mut self, command: &Command<'cmd>) -> Result<&CommandMap<'cmd>> {
        command.validate()?;
        self.check_duplicated(command)?;
        trace!(target: "cl_core::commands", alias = %command.alias, namespace = %command.namespace, "adding command");
        self.commands
            .entry(command.namespace.to_string())
            .and_modify(|commands| commands.push(command.to_owned()))
            .or_insert_with(|| vec![command.to_owned()]);

        debug!(target: "cl_core::commands", alias = %command.alias, namespace = %command.namespace, "command added");
        Ok(&self.commands)
    }

    pub fn edit(
        &mut self,
        new_command: &Command<'cmd>,
        old_command: &Command<'cmd>,
    ) -> Result<&CommandMap<'cmd>> {
        debug!(
            target: "cl_core::commands",
            old_alias = %old_command.alias, old_namespace = %old_command.namespace,
            new_alias = %new_command.alias, new_namespace = %new_command.namespace,
            "editing command"
        );
        new_command.validate()?;
        self.command_already_exists(new_command, old_command)?;
        let old_command_namespace: &str = old_command.namespace.borrow();
        let new_command_namespace = new_command.namespace.to_string();

        if let Some(commands) = self.commands.get_mut(old_command_namespace) {
            commands.retain(|command| !command.eq(old_command));
        }

        self.commands
            .entry(new_command_namespace.clone())
            .and_modify(|commands| commands.push(new_command.to_owned()))
            .or_insert_with(|| vec![new_command.to_owned()]);

        if let Some(commands) = self.commands.get(old_command_namespace) {
            if commands.is_empty() {
                trace!(target: "cl_core::commands", namespace = %old_command_namespace, "removing empty namespace after edit");
                self.commands.remove(old_command_namespace);
            }
        }

        debug!(target: "cl_core::commands", alias = %new_command.alias, namespace = %new_command_namespace, "command edited");
        Ok(&self.commands)
    }

    pub fn remove(&mut self, command: &Command) -> Result<&CommandMap<'cmd>> {
        let namespace: &str = command.namespace.borrow();
        debug!(target: "cl_core::commands", alias = %command.alias, namespace = %namespace, "removing command");

        if let Some(commands) = self.commands.get_mut(namespace) {
            commands.retain(|c| !c.eq(command));
            if commands.is_empty() {
                trace!(target: "cl_core::commands", namespace = %namespace, "removing empty namespace after remove");
                self.commands.remove(namespace);
            }
            debug!(target: "cl_core::commands", alias = %command.alias, namespace = %namespace, "command removed");
        } else {
            warn!(target: "cl_core::commands", alias = %command.alias, namespace = %namespace, "namespace not found during remove");
        }

        Ok(&self.commands)
    }

    pub fn find(&self, alias: &str, namespace: Option<&str>) -> Result<Command<'cmd>> {
        debug!(target: "cl_core::commands", alias = %alias, namespace = ?namespace, "finding command");
        let binding = self.commands.to_vec();
        let commands = match namespace {
            Some(ns) => self
                .commands
                .get(ns)
                .and_then(|cmds| cmds.iter().find(|command| command.alias == alias)),

            None => {
                let filter = binding.filter(|cmd| cmd.alias == alias);
                trace!(target: "cl_core::commands", alias = %alias, candidates = %filter.len(), "scanned all namespaces");

                if filter.len() > 1 {
                    bail!(CommandError::CommandPresentInManyNamespaces {
                        alias: alias.to_owned()
                    })
                }

                filter.into_iter().next()
            }
        };

        match commands.map(|c| c.to_owned()) {
            Some(c) => {
                debug!(target: "cl_core::commands", alias = %alias, namespace = %c.namespace, "command found");
                Ok(c)
            }
            None => bail!(CommandError::AliasNotFound {
                alias: alias.to_owned()
            }),
        }
    }

    fn check_same_alias(&self, new_command: &Command<'cmd>) -> bool {
        let namespace: &str = new_command.namespace.borrow();

        if let Some(commands) = self.commands.get(namespace) {
            commands
                .iter()
                .any(|command| command.alias.eq(&new_command.alias))
        } else {
            false
        }
    }

    fn check_duplicated(&self, new_command: &Command<'cmd>) -> Result<()> {
        if self.check_same_alias(new_command) {
            bail!(CommandError::CommandAlreadyExists {
                alias: new_command.alias.to_string(),
                namespace: new_command.namespace.to_string(),
            });
        }
        Ok(())
    }

    fn command_already_exists(&self, actual: &Command<'cmd>, old: &Command<'cmd>) -> Result<()> {
        let old_namespace = old.namespace.to_string();
        let actual_namespace = actual.namespace.to_string();

        // Identity edit (alias+namespace unchanged) is always allowed
        if old_namespace == actual_namespace && old.alias == actual.alias {
            return Ok(());
        }

        // Check if the new (alias, namespace) collides with a different existing command
        if let Some(commands) = self.get_namespace_content(&actual_namespace) {
            if commands.iter().any(|command| command.alias == actual.alias) {
                bail!(CommandError::CommandAlreadyExists {
                    alias: actual.alias.to_string(),
                    namespace: actual_namespace,
                });
            }
        }

        Ok(())
    }
}

impl<'cmd> Commands<'cmd> {
    pub fn get_namespace_content(&self, namespace: &str) -> Option<&CommandVec<'cmd>> {
        self.commands.get(namespace)
    }

    pub fn as_list(&self) -> CommandVec<'cmd> {
        self.commands.to_vec()
    }

    pub fn as_map(&self) -> &CommandMap<'cmd> {
        &self.commands
    }
}

pub trait CommandExec {
    fn exec(&self, dry_run: bool, quiet_mode: bool) -> Result<()>;

    fn truncate_command(&self) -> String;
}

impl CommandExec for Command<'_> {
    /// Executes a command
    ///
    /// If no `$SHELL` is set, defaults to `sh`
    ///
    /// ## Arguments
    /// * `command_item` - The command entity itself
    /// * `dry_run` - A boolean flag representing if the command should be actually executed or just printed in the `stdout`
    /// * `quiet_mode` - A boolean flag representing if the command string should be shown before the command output
    fn exec(&self, dry_run: bool, quiet_mode: bool) -> Result<()> {
        if dry_run {
            println!("{}", self.command);
            return Ok(());
        }

        if !quiet_mode {
            let truncated_command = self.truncate_command();
            eprintln!(
                "{}.{} --> {}",
                self.namespace, self.alias, truncated_command
            );
        }

        let shell = env::var("SHELL").unwrap_or_else(|_| {
            warn!("$SHELL not found! Using sh");
            String::from("sh")
        });

        std::process::Command::new(shell)
            .arg("-c")
            .arg(self.command.borrow() as &str)
            .spawn()?
            .wait()
            .context("The command did not run")
            .map_err(|err| CommandError::CannotRunCommand {
                command: self.command.to_string(),
                cause: err.to_string(),
            })?;

        Ok(())
    }

    fn truncate_command(&self) -> String {
        const MAX_LINE_LENGTH: usize = 120;
        let truncated: String = self.command.chars().take(MAX_LINE_LENGTH).collect();
        if truncated.len() < self.command.len() {
            format!("{truncated}...")
        } else {
            self.command.to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::CommandVecExt;
    use core::panic;
    use std::borrow::Cow;

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

    macro_rules! commands {
        ($($command:expr),+ $(,)?) => {{
            let command_map = vec![$($command),+].to_command_map();

            Commands::init(command_map)
        }};
    }

    trait PanicIfError<T> {
        fn panic_if_error(&self) -> &T;
    }

    impl<T> PanicIfError<T> for Result<T> {
        fn panic_if_error(&self) -> &T {
            match self {
                Ok(value) => value,
                Err(err) => panic!("Error: {:?}", err),
            }
        }
    }

    #[test]
    fn should_return_all_commands() {
        let command1 = create_command!("alias1", "command1", "namespace1", None, None);
        let command2 = create_command!("alias2", "command2", "namespace2", None, None);

        let commands = commands!(command1.to_owned(), command2.to_owned());
        let all_command_items = commands.as_list();
        assert_eq!(2, all_command_items.len())
    }

    #[test]
    fn should_return_an_error_when_add_a_duplicated_command() {
        let command = create_command!("alias1", "command1", "namespace1", None, None);
        let mut commands = commands!(command.to_owned());

        let duplicated = command.clone();
        let result = commands.add(&duplicated);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                CommandError::CommandAlreadyExists {
                    alias: duplicated.alias.to_string(),
                    namespace: duplicated.namespace.to_string()
                }
                .to_string()
            )
        } else {
            panic!("Error: result should be an error")
        }
    }

    #[test]
    fn should_remove_a_command() {
        let command = create_command!("alias1", "command1", "namespace1", None, None);
        let mut commands = commands!(command.to_owned());

        let commands_list = commands.as_list();
        assert_eq!(1, commands_list.len());

        let to_be_removed = commands_list.get(0).unwrap();
        let commands_after_item_removed = commands.remove(to_be_removed);

        assert!(!commands_after_item_removed
            .panic_if_error()
            .contains_key(&command.namespace.to_string()));
    }

    #[test]
    fn should_add_a_command() {
        let command = create_command!("old", "command", "namespace", None, None);
        let mut commands = commands!(command.to_owned());
        let all_commands = commands.as_list();

        assert!(!all_commands.is_empty());

        let new_command = create_command!("new", "command", "namespace", None, None);
        let commands_with_new_command = commands.add(&new_command);

        let new_command_list = commands_with_new_command
            .panic_if_error()
            .get(&new_command.namespace.to_string())
            .unwrap();
        assert!(new_command_list.contains(&command))
    }

    #[test]
    fn should_add_an_edited_command() {
        let new_command = create_command!("old", "command", "namespace", None, None);
        let mut commands = commands!(new_command.to_owned());

        let edited_command = create_command!("new", "command", "namespace", None, None);

        let old_command = new_command;
        let commands_with_edited_command = commands.edit(&edited_command, &old_command);

        assert!(commands_with_edited_command.is_ok());

        let command_map = commands_with_edited_command.panic_if_error();
        assert!(command_map.contains_key(&edited_command.namespace.to_string()));

        let entry = command_map
            .get(&edited_command.namespace.to_string())
            .unwrap();
        assert!(entry.contains(&edited_command));
        assert!(!entry.contains(&old_command));
    }

    #[test]
    fn should_add_an_edited_command_with_same_alias_but_different_namespace() {
        let new_command = create_command!("old", "command", "namespace", None, None);
        let mut commands = commands!(new_command.to_owned());

        let mut edited_command = new_command.clone();
        edited_command.namespace = Cow::Borrowed("edited_namespace");

        let old_command = new_command;
        let commands_with_edited_command = commands.edit(&edited_command, &old_command);

        assert!(commands_with_edited_command.is_ok());

        let command_map = commands_with_edited_command.panic_if_error();
        assert!(command_map.contains_key(&edited_command.namespace.to_string()));

        let entry = command_map
            .get(&edited_command.namespace.to_string())
            .unwrap();
        assert!(entry.contains(&edited_command));
        assert!(!entry.contains(&old_command));
    }

    #[test]
    fn should_return_an_error_when_add_an_edited_command_with_duplicated_alias_in_the_same_namespace(
    ) {
        let command1 = create_command!("alias1", "command", "namespace1", None, None);
        let command2 = create_command!("alias2", "command", "namespace1", None, None);
        let mut commands = commands!(command1, command2.to_owned());

        let edited_command = create_command!("alias1", "command", "namespace1", None, None);

        let command_list_with_edited_command = commands.edit(&edited_command, &command2);

        assert!(command_list_with_edited_command.is_err());
        assert_eq!(
            CommandError::CommandAlreadyExists {
                alias: edited_command.alias.to_string(),
                namespace: edited_command.namespace.to_string()
            }
            .to_string(),
            command_list_with_edited_command.unwrap_err().to_string()
        )
    }

    #[test]
    fn should_return_an_error_when_add_an_edited_command_with_duplicated_alias_and_namespace() {
        let command1 = create_command!("alias1", "command", "namespace1", None, None);
        let command2 = create_command!("alias2", "command", "namespace2", None, None);
        let mut commands = commands!(command1, command2.to_owned());

        let mut edited_command = command2.clone();
        edited_command.alias = Cow::Borrowed("alias1");
        edited_command.namespace = Cow::Borrowed("namespace1");
        let command_list_with_edited_command = commands.edit(&edited_command, &command2);

        assert!(command_list_with_edited_command.is_err());
        assert_eq!(
            CommandError::CommandAlreadyExists {
                alias: edited_command.alias.to_string(),
                namespace: edited_command.namespace.to_string()
            }
            .to_string(),
            command_list_with_edited_command.unwrap_err().to_string()
        )
    }

    #[test]
    fn should_find_a_command() {
        let command1 = create_command!("alias1", "command", "namespace1", None, None);
        let command2 = create_command!("alias2", "command", "namespace2", None, None);
        let commands = commands!(command1.to_owned(), command2.to_owned());

        let result = commands.find(&command1.alias, None);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), command1)
    }

    #[test]
    fn should_return_an_error_if_find_more_than_on_command_with_same_alias() {
        let command1 = create_command!("alias", "command", "namespace1", None, None);
        let command2 = create_command!("alias", "command", "namespace2", None, None);
        let commands = commands!(command1.to_owned(), command2.to_owned());

        let result = commands.find(&command1.alias, None);

        assert!(result.is_err());
        assert_eq!(
            CommandError::CommandPresentInManyNamespaces {
                alias: command1.alias.to_string()
            }
            .to_string(),
            result.unwrap_err().to_string()
        )
    }

    #[test]
    fn should_allow_identity_edit_with_same_alias_and_namespace() {
        let command1 = create_command!("alias", "command", "namespace1", None, None);
        let command2 = create_command!("alias", "command", "namespace1", None, None);

        let commands = commands!(command1.to_owned(), command2.to_owned());
        let res = commands.command_already_exists(&command1, &command2);

        // Identity edit (same alias+namespace) should be allowed
        assert!(res.is_ok())
    }

    #[test]
    fn should_reject_edit_when_target_alias_collides_with_different_command() {
        let command1 = create_command!("alias1", "command", "namespace1", None, None);
        let command2 = create_command!("alias2", "command", "namespace1", None, None);

        let commands = commands!(command1.to_owned(), command2.to_owned());
        // Editing command2 to have alias1's identity should fail
        let edited = create_command!("alias1", "new command", "namespace1", None, None);
        let res = commands.command_already_exists(&edited, &command2);

        assert!(res.is_err())
    }

    #[test]
    fn should_return_an_error_if_alias_does_not_exists() {
        let command1 = create_command!("alias", "command", "namespace1", None, None);
        let commands = commands!(command1);
        let invalid_alias = "invalid";
        let result = commands.find(&invalid_alias, None);

        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                CommandError::AliasNotFound {
                    alias: invalid_alias.to_owned()
                }
                .to_string(),
                error.to_string()
            )
        }
    }

    #[test]
    fn should_execute_a_command() {
        // nothing much to test here without capturing the stdout, so just checks the method output
        // note that this test function will actually run the provided command, BE CAREFUL

        let command_to_be_executed = "echo 'Hello, world!' > /dev/null 2>&1";
        let command = create_command!("alias", command_to_be_executed, "namespace1", None, None);

        // dry run
        let dry_run = true;
        let quiet_mode = false;
        let result = command.exec(dry_run, quiet_mode);
        assert!(result.is_ok());

        // dry run & quiet
        let dry_run = true;
        let quiet_mode = true;
        let result = command.exec(dry_run, quiet_mode);
        assert!(result.is_ok());

        // quiet
        let dry_run = false;
        let quiet_mode = true;
        let result = command.exec(dry_run, quiet_mode);
        assert!(result.is_ok());

        // false dry run & false quiet
        let dry_run = false;
        let quiet_mode = false;
        let result = command.exec(dry_run, quiet_mode);
        assert!(result.is_ok());
    }

    #[test]
    fn should_reject_add_with_invalid_alias() {
        let invalid = create_command!("invalid alias", "command", "namespace", None, None);
        let mut commands = Commands::default();

        let result = commands.add(&invalid);
        assert!(result.is_err());
        assert_eq!(
            CommandError::AliasWithWhitespaces.to_string(),
            result.unwrap_err().to_string()
        );
    }

    #[test]
    fn should_reject_add_with_empty_namespace() {
        let invalid = create_command!("alias", "command", "", None, None);
        let mut commands = Commands::default();

        let result = commands.add(&invalid);
        assert!(result.is_err());
        assert_eq!(
            CommandError::EmptyCommand.to_string(),
            result.unwrap_err().to_string()
        );
    }

    #[test]
    fn should_allow_editing_command_body_without_changing_alias_or_namespace() {
        let original = create_command!("alias", "old command", "namespace", None, None);
        let mut commands = commands!(original.to_owned());

        let edited = create_command!("alias", "new command", "namespace", None, None);
        let result = commands.edit(&edited, &original);

        assert!(result.is_ok());
        let map = result.panic_if_error();
        let entry = map.get("namespace").unwrap();
        assert!(entry.iter().any(|c| c.command == "new command"));
    }

    #[test]
    fn should_reject_edit_with_invalid_alias() {
        let original = create_command!("alias", "command", "namespace", None, None);
        let mut commands = commands!(original.to_owned());

        let invalid_new = create_command!("new alias", "command", "namespace", None, None);
        let result = commands.edit(&invalid_new, &original);
        assert!(result.is_err());
        assert_eq!(
            CommandError::AliasWithWhitespaces.to_string(),
            result.unwrap_err().to_string()
        );
    }
}
