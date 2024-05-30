use crate::{
    command::Command, resource::errors::CommandError, CommandMap, CommandMapExt, CommandVec,
};
use anyhow::{bail, Context, Result};
use std::env;

#[derive(Default)]
pub struct Commands {
    commands: CommandMap,
}

impl Commands {
    pub fn init(commands: CommandMap) -> Commands {
        Commands { commands }
    }

    pub fn commands(&self) -> CommandMap {
        self.commands.to_owned()
    }

    pub fn command_as_list(&self) -> CommandVec {
        self.commands.to_vec()
    }

    pub fn add_command(&mut self, command: &Command) -> Result<CommandMap> {
        self.check_duplicated(command)?;
        log::debug!("commands before insert: {:?}", self.commands);
        self.commands
            .entry(command.namespace.to_owned())
            .and_modify(|commands| commands.push(command.to_owned()))
            .or_insert_with(|| vec![command.to_owned()]);

        log::debug!("commands after insert: {:?}", self.commands);
        Ok(self.commands.to_owned())
    }

    pub fn add_edited_command(
        &mut self,
        new_command: &Command,
        old_command: &Command,
    ) -> Result<CommandMap> {
        self.compare_edited_command(new_command, old_command)?;

        if let Some(commands) = self.commands.get_mut(&old_command.namespace) {
            commands.retain(|command| !command.eq(old_command));
        }

        self.commands
            .entry(new_command.namespace.to_owned())
            .and_modify(|commands| commands.push(new_command.to_owned()))
            .or_insert_with(|| vec![new_command.to_owned()]);

        if let Some(commands) = self.commands.get(&old_command.namespace) {
            if commands.is_empty() {
                self.commands.remove(&old_command.namespace);
            }
        }

        Ok(self.commands.to_owned())
    }

    pub fn remove(&mut self, command: &Command) -> Result<CommandMap> {
        if let Some(commands) = self.commands.get_mut(&command.namespace) {
            if commands.len() <= 1 {
                self.commands.remove(&command.namespace);
            } else {
                commands.retain(|c| !c.eq(command));
            }
        };

        Ok(self.commands.to_owned())
    }

    /// Executes a command
    ///
    /// If no `$SHELL` is set, defaults to `sh`
    ///
    /// ## Arguments
    /// * `command_item` - The command entity itself
    /// * `dry_run` - A boolean flag representing if the command should be actually executed or just printed in the `stdout`
    /// * `quiet_mode` - A boolean flag representing if the command string should be shown before the command output
    pub fn exec_command(
        &self,
        command_item: &Command,
        dry_run: bool,
        quiet_mode: bool,
    ) -> Result<()> {
        if dry_run {
            println!("{}", command_item.command);
        } else {
            if !quiet_mode {
                const MAX_LINE_LENGTH: usize = 120;
                let command_description = if command_item.command.len() > MAX_LINE_LENGTH {
                    format!(
                        "{}{}",
                        &command_item.command.clone()[..MAX_LINE_LENGTH],
                        "..."
                    )
                } else {
                    command_item.command.clone()
                };
                eprintln!(
                    "{}.{} --> {}",
                    command_item.namespace, command_item.alias, command_description
                );
            }

            let shell = env::var("SHELL").unwrap_or_else(|_| {
                eprintln!("Warning: $SHELL not found! Using sh");
                String::from("sh")
            });

            std::process::Command::new(shell)
                .env_clear()
                .envs(env::vars())
                .arg("-c")
                .arg(&command_item.command)
                .spawn()?
                .wait()
                .context("The command did not run")?;
        }
        Ok(())
    }

    pub fn find_command(&self, alias: &str, namespace: Option<String>) -> Result<Command> {
        let commands = if let Some(namespace) = namespace {
            if let Some(commands) = self.commands.get(&namespace) {
                commands
                    .iter()
                    .filter(|command| command.alias.eq(&alias))
                    .map(|c| c.to_owned())
                    .collect::<CommandVec>()
            } else {
                CommandVec::new()
            }
        } else {
            self.commands
                .to_vec()
                .iter()
                .filter(|command| {
                    namespace
                        .as_ref()
                        .map_or(true, |ns| command.namespace.eq(ns))
                        && command.alias.eq(&alias)
                })
                .map(|c| c.to_owned())
                .collect::<CommandVec>()
        };

        if commands.is_empty() {
            bail!(CommandError::AliasNotFound {
                alias: alias.to_owned()
            })
        } else if commands.len() == 1 {
            Ok(commands[0].to_owned())
        } else {
            bail!(CommandError::CommandPresentInManyNamespaces {
                alias: alias.to_owned()
            })
        }
    }

    fn check_same_alias(&self, new_command: &Command) -> bool {
        if let Some(commands) = self.commands.get(&new_command.namespace) {
            commands
                .iter()
                .any(|command| command.alias.eq(&new_command.alias))
        } else {
            false
        }
    }

    fn check_duplicated(&self, new_command: &Command) -> Result<()> {
        if self.check_same_alias(new_command) {
            bail!(CommandError::CommandAlreadyExists {
                alias: new_command.alias.to_owned(),
                namespace: new_command.namespace.to_owned(),
            });
        }
        Ok(())
    }

    fn compare_edited_command(&self, new_command: &Command, old_command: &Command) -> Result<()> {
        let same_alias = self.check_same_alias(new_command);
        let has_changed = new_command.has_changes(old_command);
        if same_alias || !has_changed {
            bail!(CommandError::CommandAlreadyExists {
                alias: new_command.alias.to_owned(),
                namespace: new_command.namespace.to_owned()
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::CommandVecExt;
    use core::panic;

    macro_rules! create_command {
        ($alias:expr, $command:expr, $namespace:expr, $description:expr, $tags:expr) => {
            Command {
                alias: $alias.to_owned(),
                namespace: $namespace.to_owned(),
                command: $command.to_owned(),
                description: $description,
                tags: $tags,
            }
        };
    }

    macro_rules! commands {
        ($commands:expr) => {
            Commands::init($commands)
        };
    }

    #[test]
    fn should_return_all_commands() {
        let command1 = create_command!("alias1", "command1", "namespace1", None, None);
        let command2 = create_command!("alias2", "command2", "namespace2", None, None);

        let commands = commands!(vec![command1.to_owned(), command2.to_owned()].to_command_map());
        let all_command_items = commands.command_as_list();
        assert_eq!(2, all_command_items.len())
    }

    #[test]
    fn should_return_an_error_when_add_a_duplicated_command() {
        let command = create_command!("alias1", "command1", "namespace1", None, None);
        let mut commands = commands!(vec![command.to_owned()].to_command_map());

        let duplicated = command.clone();
        let result = commands.add_command(&duplicated);
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(
                error.to_string(),
                CommandError::CommandAlreadyExists {
                    alias: duplicated.alias,
                    namespace: duplicated.namespace
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
        let mut commands = commands!(vec![command.to_owned()].to_command_map());

        let commands_list = commands.command_as_list();
        assert_eq!(1, commands_list.len());

        let to_be_removed = commands_list.get(0).unwrap();
        let commands_after_item_removed = commands.remove(to_be_removed);

        assert!(commands_after_item_removed.is_ok());
        if let Ok(commands_map) = commands_after_item_removed {
            assert!(!commands_map.contains_key(&command.namespace));
        } else {
            panic!("Error: {:?}", commands_after_item_removed.unwrap_err());
        }
    }

    #[test]
    fn should_add_a_command() {
        let command = create_command!("old", "command", "namespace", None, None);
        let mut commands = commands!(vec![command.to_owned()].to_command_map());
        let all_commands = commands.command_as_list();

        assert!(!all_commands.is_empty());
        let new_command = create_command!("new", "command", "namespace", None, None);
        let commands_with_new_command = commands.add_command(&new_command);

        assert!(commands_with_new_command.is_ok());
        if let Ok(commands) = commands_with_new_command {
            let new_command_list = commands.get(&new_command.namespace).unwrap();
            assert!(new_command_list.contains(&command))
        } else {
            panic!("Error: {:?}", commands_with_new_command.unwrap_err());
        }
    }

    #[test]
    fn should_add_an_edited_command() {
        let new_command = create_command!("old", "command", "namespace", None, None);
        let mut commands = commands!(vec![new_command.to_owned()].to_command_map());

        let mut edited_command = new_command.clone();
        edited_command.alias = String::from("new");

        let old_command = new_command;
        let commands_with_edited_command =
            commands.add_edited_command(&edited_command, &old_command);

        assert!(commands_with_edited_command.is_ok());
        if let Ok(command_map) = commands_with_edited_command {
            assert!(command_map.contains_key(&edited_command.namespace));
            let entry = command_map.get(&edited_command.namespace).unwrap();
            assert!(entry.contains(&edited_command));
            assert!(!entry.contains(&old_command));
        } else {
            panic!("Error: {:?}", commands_with_edited_command.unwrap_err());
        }
    }

    #[test]
    fn should_add_an_edited_command_with_same_alias_but_different_namespace() {
        let new_command = create_command!("old", "command", "namespace", None, None);
        let mut commands = commands!(vec![new_command.to_owned()].to_command_map());

        let mut edited_command = new_command.clone();
        edited_command.namespace = String::from("edited_namespace");

        let old_command = new_command;
        let commands_with_edited_command =
            commands.add_edited_command(&edited_command, &old_command);

        assert!(commands_with_edited_command.is_ok());
        if let Ok(command_map) = commands_with_edited_command {
            assert!(command_map.contains_key(&edited_command.namespace));
            let entry = command_map.get(&edited_command.namespace).unwrap();
            assert!(entry.contains(&edited_command));
            assert!(!entry.contains(&old_command));
        } else {
            panic!("Error: {:?}", commands_with_edited_command.unwrap_err());
        }
    }

    #[test]
    fn should_return_an_error_when_add_an_edited_command_with_duplicated_alias_in_the_same_namespace(
    ) {
        let command1 = create_command!("alias1", "command", "namespace1", None, None);
        let command2 = create_command!("alias2", "command", "namespace1", None, None);
        let mut commands = commands!(vec![command1, command2.to_owned()].to_command_map());

        let mut edited_command = command2.clone();
        edited_command.alias = String::from("alias1");

        let command_list_with_edited_command =
            commands.add_edited_command(&edited_command, &command2);

        assert!(command_list_with_edited_command.is_err());
        assert_eq!(
            CommandError::CommandAlreadyExists {
                alias: edited_command.alias,
                namespace: edited_command.namespace
            }
            .to_string(),
            command_list_with_edited_command.unwrap_err().to_string()
        )
    }

    #[test]
    fn should_return_an_error_when_add_an_edited_command_with_duplicated_alias_and_namespace() {
        let command1 = create_command!("alias1", "command", "namespace1", None, None);
        let command2 = create_command!("alias2", "command", "namespace2", None, None);
        let mut commands = commands!(vec![command1, command2.to_owned()].to_command_map());

        let mut edited_command = command2.clone();
        edited_command.alias = String::from("alias1");
        edited_command.namespace = String::from("namespace1");
        let command_list_with_edited_command =
            commands.add_edited_command(&edited_command, &command2);

        assert!(command_list_with_edited_command.is_err());
        assert_eq!(
            CommandError::CommandAlreadyExists {
                alias: edited_command.alias,
                namespace: edited_command.namespace
            }
            .to_string(),
            command_list_with_edited_command.unwrap_err().to_string()
        )
    }

    #[test]
    fn should_find_a_command() {
        let command1 = create_command!("alias1", "command", "namespace1", None, None);
        let command2 = create_command!("alias2", "command", "namespace2", None, None);
        let commands = commands!(vec![command1.to_owned(), command2.to_owned()].to_command_map());

        let result = commands.find_command(&command1.alias, None);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), command1)
    }

    #[test]
    fn should_return_an_error_if_find_more_than_on_command_with_same_alias() {
        let command1 = create_command!("alias", "command", "namespace1", None, None);
        let command2 = create_command!("alias", "command", "namespace2", None, None);
        let commands = commands!(vec![command1.to_owned(), command2.to_owned()].to_command_map());

        let result = commands.find_command(&command1.alias, None);

        assert!(result.is_err());
        assert_eq!(
            CommandError::CommandPresentInManyNamespaces {
                alias: command1.alias
            }
            .to_string(),
            result.unwrap_err().to_string()
        )
    }

    #[test]
    fn should_return_an_error_if_alias_does_not_exists() {
        let command1 = create_command!("alias", "command", "namespace1", None, None);
        let commands = commands!(vec![command1].to_command_map());
        let invalid_alias = "invalid";
        let result = commands.find_command(&invalid_alias, None);

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

    // #[test]
    // fn should_execute_a_command() {
    //     // nothing much to test here without capturing the stdout, so just checks the method output
    //     // note that this test function will actually run the provided command, BE CAREFUL
    //     let commands = build_commands();
    //     let mut command_be_executed = commands.commands[0].clone();
    //     command_be_executed.command = "echo 'Hello, world!' > /dev/null 2>&1".to_owned();

    //     // dry run
    //     let dry_run = true;
    //     let quiet_mode = false;
    //     let result = commands.exec_command(&command_be_executed, dry_run, quiet_mode);
    //     assert!(result.is_ok());

    //     // dry run & quiet
    //     let dry_run = true;
    //     let quiet_mode = true;
    //     let result = commands.exec_command(&command_be_executed, dry_run, quiet_mode);
    //     assert!(result.is_ok());

    //     // quiet
    //     let dry_run = false;
    //     let quiet_mode = true;
    //     let result = commands.exec_command(&command_be_executed, dry_run, quiet_mode);
    //     assert!(result.is_ok());

    //     // false dry run & false quiet
    //     let dry_run = false;
    //     let quiet_mode = false;
    //     let result = commands.exec_command(&command_be_executed, dry_run, quiet_mode);
    //     assert!(result.is_ok());

    //     command_be_executed.command =
    //         "echo 'a very looooooooooooooooooooooooooooooooooooooooooooooooooo
    //     ooooooooooooooooooooooooooooooooooooooooooooong command' > /dev/null 2>&1"
    //             .to_owned();

    //     // false dry run & false quiet
    //     let dry_run = false;
    //     let quiet_mode = false;
    //     let result = commands.exec_command(&command_be_executed, dry_run, quiet_mode);
    //     assert!(result.is_ok());
    // }
}
