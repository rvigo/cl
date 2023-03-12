use crate::{command::Command, resources::errors::CommandError};
use anyhow::{bail, ensure, Result};
use std::env;

#[derive(Default)]
pub struct Commands {
    commands: Vec<Command>,
}

impl Commands {
    pub fn init(mut commands: Vec<Command>) -> Commands {
        commands.sort_by_key(|c| c.alias.to_lowercase());
        Commands { commands }
    }

    pub fn command_list(&self) -> &Vec<Command> {
        &self.commands
    }

    pub fn add_command(&mut self, command: &Command) -> Result<Vec<Command>> {
        ensure!(
            !self.command_already_exists(command),
            CommandError::CommandAlreadyExists {
                alias: command.alias.to_owned(),
                namespace: command.namespace.to_owned()
            }
        );
        self.commands.push(command.to_owned());
        self.commands.sort_by_key(|c| c.alias.to_lowercase());
        Ok(self.commands.to_owned())
    }

    pub fn add_edited_command(
        &mut self,
        edited_command: &Command,
        old_command: &Command,
    ) -> Result<Vec<Command>> {
        ensure!(
            !self.commands.clone().iter().any(|command| {
                command.alias.eq(&edited_command.alias)
                    && !edited_command.alias.eq(&old_command.alias)
                    && command.namespace.eq(&edited_command.namespace)
            }),
            CommandError::CommandAlreadyExists {
                alias: edited_command.alias.to_owned(),
                namespace: edited_command.namespace.to_owned()
            }
        );

        self.commands.retain(|command| !command.eq(old_command));
        self.commands.push(edited_command.to_owned());
        self.commands.sort_by_key(|c| c.alias.to_lowercase());

        Ok(self.commands.to_owned())
    }

    pub fn remove(&mut self, command: &Command) -> Result<Vec<Command>> {
        self.commands
            .retain(|c| !c.alias.eq(&command.alias) || !command.namespace.eq(&c.namespace));
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
        const MAX_LINE_LENGTH: usize = 120;

        let shell = env::var("SHELL").unwrap_or_else(|_| {
            eprintln!("Warning: $SHELL not found! Using sh");
            String::from("sh")
        });
        // let envs = env::vars();

        if dry_run {
            println!("{}", command_item.command);
        } else {
            if !quiet_mode {
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
            std::process::Command::new(shell)
                .env_clear()
                .envs(env::vars())
                .arg("-ic")
                .arg(&command_item.command)
                .spawn()?
                .wait()
                .expect("The command did not run");
        }
        Ok(())
    }

    pub fn find_command(&self, alias: String, namespace: Option<String>) -> Result<Command> {
        let commands: Vec<Command> = self
            .commands
            .iter()
            .cloned()
            .filter(|command| {
                namespace
                    .as_ref()
                    .map_or(true, |ns| command.namespace.eq(ns))
                    && command.alias.eq(&alias)
            })
            .collect();

        if commands.is_empty() {
            bail!(CommandError::AliasNotFound { alias })
        } else if commands.len() == 1 {
            Ok(commands[0].to_owned())
        } else {
            bail!(CommandError::CommandPresentInManyNamespaces { alias })
        }
    }

    fn command_already_exists(&self, command_item: &Command) -> bool {
        self.commands.iter().any(|command| {
            command.alias == command_item.alias && command.namespace.eq(&command_item.namespace)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::command::CommandBuilder;

    fn command_factory(
        alias: &str,
        namespace: &str,
        command: &str,
        tags: Option<Vec<&str>>,
        description: Option<&str>,
    ) -> Command {
        let mut builder = CommandBuilder::default();
        builder
            .alias(String::from(alias))
            .namespace(String::from(namespace))
            .command(String::from(command))
            .description(description.map(String::from))
            .tags(tags.map(|v| v.into_iter().map(String::from).collect::<Vec<String>>()));
        builder.build()
    }
    fn build_commands() -> Commands {
        let command1 = command_factory(
            "alias1",
            "namespace1",
            "command1",
            Some(vec!["tag1", "tag2"]),
            Some("description"),
        );

        let command2 = command_factory(
            "alias2",
            "namespace2",
            "command2",
            Some(vec!["tag1", "tag2"]),
            Some("description"),
        );

        Commands::init(vec![command1, command2])
    }

    #[test]
    fn should_return_all_commands() {
        let commands = build_commands();
        let all_command_items = commands.command_list();
        assert_eq!(2, all_command_items.len())
    }

    #[test]
    fn should_validate_if_command_already_exists() {
        let commands = build_commands();
        let mut duplicated_command = CommandBuilder::default();
        duplicated_command
            .alias(String::from("alias1"))
            .namespace(String::from("namespace1"))
            .command(String::from("command"));

        let already_exists = commands.command_already_exists(&duplicated_command.build());
        assert!(already_exists)
    }

    #[test]
    fn should_remove_a_command() {
        let mut commands = build_commands();
        let all_commands = commands.command_list().to_owned();

        assert_eq!(2, all_commands.len());

        let to_be_removed = all_commands.get(0).unwrap();
        let command_list_after_remove_command = commands.remove(to_be_removed);

        if let Ok(items) = command_list_after_remove_command {
            assert_eq!(1, items.len());
            assert!(!items.contains(to_be_removed))
        }
    }

    #[test]
    fn should_add_a_command() {
        let mut commands = build_commands();
        let all_commands = commands.command_list();

        assert_eq!(2, all_commands.len());

        let new_command = Command::default();
        let new_command_list = commands.add_command(&new_command);

        if let Ok(items) = new_command_list {
            assert_eq!(3, items.len());
            assert!(items.contains(&new_command))
        }
    }

    #[test]
    fn should_add_an_edited_command() {
        let mut commands = build_commands();
        let new_command = command_factory(
            "alias2",
            "namespace1",
            "command2",
            Some(vec!["tag1", "tag2"]),
            Some("description"),
        );

        if let Ok(command_list) = commands.add_command(&new_command) {
            assert_eq!(3, command_list.len())
        }

        let mut edited_command = new_command.clone();
        edited_command.alias = String::from("edited_alias");

        let command_list_with_edited_command =
            commands.add_edited_command(&edited_command, &new_command);

        if let Ok(command_list) = command_list_with_edited_command {
            assert_eq!(3, command_list.len());
            assert!(command_list.contains(&edited_command));
            assert!(!command_list.contains(&new_command));
        }
    }

    #[test]
    fn should_return_an_error_when_edit_a_duplicated_alias_command() {
        let mut commands = build_commands();
        let current_command = Command::default();

        commands.add_command(&current_command).unwrap();

        assert_eq!(3, commands.command_list().len());

        let mut edited_command = current_command.clone();
        edited_command.description = Some(String::from("edited command"));
        edited_command.namespace = String::from("namespace1");

        let command_list_with_edited_command =
            commands.add_edited_command(&edited_command, &current_command);

        if let Err(error) = command_list_with_edited_command {
            assert_eq!(
                CommandError::CommandAlreadyExists {
                    alias: edited_command.alias,
                    namespace: edited_command.namespace
                }
                .to_string(),
                error.to_string()
            )
        }
    }
}
