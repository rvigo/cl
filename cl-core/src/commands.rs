use crate::{
    command::Command, resource::errors::CommandError, CommandMap, CommandMapExt, CommandVec,
};
use anyhow::{bail, ensure, Context, Result};
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
        ensure!(
            !self.command_already_exists(command),
            CommandError::CommandAlreadyExists {
                alias: command.alias.to_owned(),
                namespace: command.namespace.to_owned()
            }
        );

        self.commands
            .entry(command.namespace.to_owned())
            .or_insert_with(|| vec![command.to_owned()]);

        Ok(self.commands.to_owned())
    }

    pub fn add_edited_command(
        &mut self,
        new_command: &Command,
        old_command: &Command,
    ) -> Result<CommandMap> {
        if let Some(commands) = self.commands.get_mut(&new_command.namespace) {
            let same_alias = commands
                .iter()
                .any(|command| command.alias.eq(&new_command.alias));

            let has_changed = old_command.has_changes(new_command);

            ensure!(
                !same_alias || has_changed,
                CommandError::CommandAlreadyExists {
                    alias: new_command.alias.to_owned(),
                    namespace: new_command.namespace.to_owned()
                }
            );
        }

        if let Some(commands) = self.commands.get_mut(&old_command.namespace) {
            commands.retain(|command| !command.eq(old_command))
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

    pub fn find_command(&self, alias: String, namespace: Option<String>) -> Result<Command> {
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
            bail!(CommandError::AliasNotFound { alias })
        } else if commands.len() == 1 {
            Ok(commands[0].to_owned())
        } else {
            bail!(CommandError::CommandPresentInManyNamespaces { alias })
        }
    }

    fn command_already_exists(&self, command_item: &Command) -> bool {
        if let Some(commands) = self.commands.get(&command_item.namespace) {
            commands.iter().any(|command| {
                command.alias == command_item.alias && command.namespace == command_item.namespace
            })
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    // use crate::command::CommandBuilder;

    // fn command_factory(
    //     alias: &str,
    //     namespace: &str,
    //     command: &str,
    //     tags: Option<Vec<&str>>,
    //     description: Option<&str>,
    // ) -> Command {
    //     let mut builder = CommandBuilder::default();
    //     builder
    //         .alias(String::from(alias))
    //         .namespace(String::from(namespace))
    //         .command(String::from(command))
    //         .description(description.map(String::from))
    //         .tags(tags.map(|v| v.into_iter().map(String::from).collect::<Vec<String>>()));
    //     builder.build()
    // }

    // fn build_commands() -> Commands {
    //     let command1 = command_factory(
    //         "alias1",
    //         "namespace1",
    //         "command1",
    //         Some(vec!["tag1", "tag2"]),
    //         Some("description"),
    //     );

    //     let command2 = command_factory(
    //         "alias2",
    //         "namespace2",
    //         "command2",
    //         Some(vec!["tag1", "tag2"]),
    //         Some("description"),
    //     );

    //     Commands::init(vec![command1, command2])
    // }

    // #[test]
    // fn should_return_all_commands() {
    //     let commands = build_commands();
    //     let all_command_items = commands.command_list();
    //     assert_eq!(2, all_command_items.len())
    // }

    // #[test]
    // fn should_return_an_error_when_add_a_duplicated_command() {
    //     let mut commands = build_commands();
    //     let mut duplicated_command_builder = CommandBuilder::default();
    //     duplicated_command_builder
    //         .alias(String::from("alias1"))
    //         .namespace(String::from("namespace1"))
    //         .command(String::from("command1"));
    //     let duplicated_command = duplicated_command_builder.build();
    //     let result = commands.add_command(&duplicated_command);
    //     assert!(result.is_err());
    //     if let Err(error) = result {
    //         assert_eq!(
    //             error.to_string(),
    //             CommandError::CommandAlreadyExists {
    //                 alias: duplicated_command.alias,
    //                 namespace: duplicated_command.namespace
    //             }
    //             .to_string()
    //         )
    //     }
    // }

    // #[test]
    // fn should_remove_a_command() {
    //     let mut commands = build_commands();
    //     let all_commands = commands.command_list().to_owned();

    //     assert_eq!(2, all_commands.len());

    //     let to_be_removed = all_commands.get(0).unwrap();
    //     let command_list_after_remove_command = commands.remove(to_be_removed);

    //     assert!(command_list_after_remove_command.is_ok());
    //     if let Ok(items) = command_list_after_remove_command {
    //         assert_eq!(1, items.len());
    //         assert!(!items.contains(to_be_removed))
    //     }
    // }

    // #[test]
    // fn should_add_a_command() {
    //     let mut commands = build_commands();
    //     let all_commands = commands.command_list();

    //     assert_eq!(2, all_commands.len());

    //     let new_command = Command::default();
    //     let new_command_list = commands.add_command(&new_command);

    //     assert!(new_command_list.is_ok());
    //     if let Ok(items) = new_command_list {
    //         assert_eq!(3, items.len());
    //         assert!(items.contains(&new_command))
    //     }
    // }

    // #[test]
    // fn should_add_an_edited_command() {
    //     let mut commands = build_commands();
    //     let new_command = command_factory(
    //         "alias2",
    //         "namespace1",
    //         "command2",
    //         Some(vec!["tag1", "tag2"]),
    //         Some("description"),
    //     );

    //     assert!(commands.add_command(&new_command).is_ok());

    //     let mut edited_command = new_command.clone();
    //     edited_command.alias = String::from("edited_alias");

    //     let command_list_with_edited_command =
    //         commands.add_edited_command(&edited_command, &new_command);

    //     assert!(command_list_with_edited_command.is_ok());
    //     if let Ok(command_list) = command_list_with_edited_command {
    //         assert!(command_list.contains(&edited_command));
    //         assert!(!command_list.contains(&new_command));
    //     }
    // }

    // #[test]
    // fn should_add_an_edited_command_with_same_alias_but_different_namespace() {
    //     let mut commands = build_commands();
    //     let new_command = command_factory(
    //         "alias2",
    //         "namespace1",
    //         "command2",
    //         Some(vec!["tag1", "tag2"]),
    //         Some("description"),
    //     );

    //     assert!(commands.add_command(&new_command).is_ok());

    //     let mut edited_command = new_command.clone();
    //     edited_command.namespace = String::from("edited_namespace");

    //     let command_list_with_edited_command =
    //         commands.add_edited_command(&edited_command, &new_command);

    //     assert!(command_list_with_edited_command.is_ok());
    //     if let Ok(command_list) = command_list_with_edited_command {
    //         assert!(command_list.contains(&edited_command));
    //         assert!(!command_list.contains(&new_command));
    //     }
    // }

    // #[test]
    // fn should_return_an_error_when_add_an_edited_command_with_duplicated_alias_in_the_same_namespace(
    // ) {
    //     let mut commands = build_commands();
    //     let mut current_command = commands.commands[0].clone();
    //     current_command.namespace = String::from("namespace2");

    //     assert_eq!(current_command.alias, "alias1");
    //     assert_eq!(current_command.namespace, "namespace2");

    //     commands.add_command(&current_command).unwrap();

    //     let mut edited_command = current_command.clone();
    //     edited_command.alias = String::from("alias2");

    //     let command_list_with_edited_command =
    //         commands.add_edited_command(&edited_command, &current_command);

    //     assert!(command_list_with_edited_command.is_err());
    //     assert_eq!(
    //         CommandError::CommandAlreadyExists {
    //             alias: edited_command.alias,
    //             namespace: edited_command.namespace
    //         }
    //         .to_string(),
    //         command_list_with_edited_command.unwrap_err().to_string()
    //     )
    // }

    // #[test]
    // fn should_return_an_error_when_add_an_edited_command_with_duplicated_alias_and_namespace() {
    //     let mut commands = build_commands();
    //     let current_command = Command::default();

    //     commands.add_command(&current_command).unwrap();

    //     let mut edited_command = current_command.clone();
    //     edited_command.alias = String::from("alias1");
    //     edited_command.namespace = String::from("namespace1");
    //     let command_list_with_edited_command =
    //         commands.add_edited_command(&edited_command, &current_command);

    //     assert!(command_list_with_edited_command.is_err());
    //     assert_eq!(
    //         CommandError::CommandAlreadyExists {
    //             alias: edited_command.alias,
    //             namespace: edited_command.namespace
    //         }
    //         .to_string(),
    //         command_list_with_edited_command.unwrap_err().to_string()
    //     )
    // }

    // #[test]
    // fn should_find_a_command() {
    //     let commands = build_commands();
    //     let first_stored_command = commands.commands[0].clone();

    //     let result = commands.find_command("alias1".to_string(), None);

    //     assert!(result.is_ok());
    //     assert_eq!(result.unwrap(), first_stored_command)
    // }

    // #[test]
    // fn should_return_an_error_if_find_more_than_on_command_with_same_alias() {
    //     let mut commands = build_commands();
    //     let target_alias = "alias1";
    //     let mut command_builder = CommandBuilder::default();
    //     command_builder
    //         .alias(target_alias)
    //         .namespace("other_namespace");
    //     let new_command = command_builder.build();

    //     assert!(commands.add_command(&new_command).is_ok());

    //     let result = commands.find_command(target_alias.to_string(), None);

    //     assert!(result.is_err());
    //     assert_eq!(
    //         CommandError::CommandPresentInManyNamespaces {
    //             alias: target_alias.to_owned()
    //         }
    //         .to_string(),
    //         result.unwrap_err().to_string()
    //     )
    // }

    // #[test]
    // fn should_return_an_error_if_alias_does_not_exists() {
    //     let commands = build_commands();

    //     let result = commands.find_command("non existent alis".to_owned(), None);

    //     assert!(result.is_err());
    //     if let Err(error) = result {
    //         assert_eq!(
    //             CommandError::AliasNotFound {
    //                 alias: "non existent alis".to_owned()
    //             }
    //             .to_string(),
    //             error.to_string()
    //         )
    //     }
    // }

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
