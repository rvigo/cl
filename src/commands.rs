use crate::command::Command;
use crate::resources::config::CONFIG;
use anyhow::{bail, ensure, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::env;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Commands(Vec<Command>);

impl Commands {
    pub fn init(items: Vec<Command>) -> Commands {
        Commands(items)
    }

    pub fn get_command_item_ref(&self, idx: usize) -> Option<&Command> {
        self.get(idx)
    }

    pub fn namespaces(&self) -> Vec<String> {
        let mut namespaces: Vec<String> = self
            .iter()
            .map(|command| command.namespace.clone())
            .unique()
            .collect();
        namespaces.insert(0, String::from("All"));
        namespaces.sort();
        namespaces
    }

    pub fn commands(&self, namespace: String, query_string: String) -> Result<Commands> {
        if self.is_empty() {
            return Ok(Commands(vec![Command::default()]));
        }
        let mut commands: Commands = self
            .iter()
            .filter(|command| self.commands_by_namespace_predicate(namespace.clone(), command))
            .filter(|command| {
                self.commands_by_query_string_predicate(query_string.clone(), command)
            })
            .map(|command| command.to_owned())
            .collect();

        ensure!(
            !commands.is_empty(),
            "There are no commands to show for namespace \"{}\"",
            namespace
        );

        commands.sort_by_key(|command| command.alias.clone());

        Ok(commands)
    }

    pub fn add_command(&mut self, command: &Command) -> Result<&Commands> {
        ensure!(
            !self.command_already_exists(command),
            "Command with alias \"{}\" already exists in \"{}\" namespace",
            command.alias,
            command.namespace
        );

        self.push(command.clone());
        Ok(self)
    }

    pub fn add_edited_command(
        &mut self,
        edited_command: &Command,
        current_command: &Command,
    ) -> Result<&Commands> {
        ensure!(
            !self.clone().iter().any(|command| {
                command.alias.eq(&edited_command.alias)
                    && !edited_command.alias.eq(&current_command.alias)
                    && command.namespace.eq(&edited_command.namespace)
            }),
            "Command with alias \"{}\" already exists in \"{}\" namespace",
            edited_command.alias,
            edited_command.namespace
        );

        self.retain(|command| command != current_command);

        self.push(edited_command.clone());
        Ok(self)
    }

    pub fn remove(&mut self, command: &Command) -> Result<&Commands> {
        self.retain(|c| !c.alias.eq(&command.alias) || !command.namespace.eq(&c.namespace));
        Ok(self)
    }

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
        if dry_run {
            println!("{}", command_item.command);
        } else {
            if !CONFIG.quiet_mode() && !quiet_mode {
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
                .arg("-c")
                .arg(&command_item.command)
                .spawn()?
                .wait()
                .expect("The command did not run");
        }
        Ok(())
    }

    pub fn find_command(&self, alias: String, namespace: Option<String>) -> Result<Command> {
        let commands: Vec<Command> = self
            .iter()
            .filter(|command| {
                namespace.is_none() || command.namespace.eq(namespace.as_ref().unwrap())
            })
            .filter(|command| command.alias.eq(&alias))
            .map(|command| command.to_owned())
            .collect();

        if commands.len() > 1 {
            bail!(
                "There are commands with the alias \'{alias}\' in multiples namespaces. \
            Please use the \'--namespace\' flag"
            )
        } else if commands.is_empty() {
            bail!("The alias \'{alias}\' was not found!")
        } else {
            Ok(commands.first().unwrap().to_owned())
        }
    }

    fn command_already_exists(&self, command_item: &Command) -> bool {
        self.iter().any(|command| {
            command.alias == command_item.alias && command.namespace.eq(&command_item.namespace)
        })
    }

    fn commands_by_query_string_predicate(
        &self,
        mut query_string: String,
        command: &Command,
    ) -> bool {
        query_string.is_empty() || {
            query_string = query_string.to_lowercase();
            command.namespace.to_lowercase().contains(&query_string)
                || command.alias.to_lowercase().contains(&query_string)
                || command.command.to_lowercase().contains(&query_string)
                || command
                    .tags_as_string()
                    .to_lowercase()
                    .contains(&query_string)
                || (command.description.is_some()
                    && command
                        .description
                        .as_ref()
                        .unwrap()
                        .to_lowercase()
                        .contains(&query_string))
        }
    }

    fn commands_by_namespace_predicate(&self, namespace: String, command: &Command) -> bool {
        namespace.eq(&String::from("All")) || command.namespace.eq(&namespace)
    }
}

impl FromIterator<Command> for Commands {
    fn from_iter<T: IntoIterator<Item = Command>>(iter: T) -> Self {
        let mut c = Commands::default();
        iter.into_iter().sorted().for_each(|i| c.push(i));
        c
    }
}

impl IntoIterator for Commands {
    type Item = Command;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Vec<Command>> for Commands {
    fn from(command_vec: Vec<Command>) -> Self {
        Commands(command_vec)
    }
}

impl Deref for Commands {
    type Target = Vec<Command>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Commands {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
            .tags(tags.map(|v| v.into_iter().map(String::from).collect_vec()));
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
    fn should_return_all_namespaces() {
        let commands = build_commands();
        let namespaces = commands.namespaces();
        assert_eq!(
            vec![
                String::from("All"),
                String::from("namespace1"),
                String::from("namespace2")
            ],
            namespaces
        )
    }

    #[test]
    fn should_return_all_commands() {
        let commands = build_commands();
        let all_command_items = commands.commands(String::from("All"), String::from(""));
        assert_eq!(2, all_command_items.unwrap().len())
    }

    #[test]
    fn should_return_welcome_command_when_there_is_no_saved_command() {
        let commands = Commands::init(Vec::default());
        let all_command_items = commands
            .commands(String::from("All"), String::from(""))
            .unwrap();
        let default_command_item = Command::default();
        assert_eq!(all_command_items.len(), 1);
        assert_eq!(
            default_command_item,
            all_command_items.get(0).unwrap().to_owned()
        );
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
        assert_eq!(true, already_exists)
    }

    #[test]
    fn should_return_all_commands_from_namespace() {
        let commands = build_commands();
        let commands_from_namespace =
            commands.commands(String::from("namespace2"), String::from(""));

        if let Ok(items) = commands_from_namespace {
            assert_eq!(1, items.len())
        }
    }

    #[test]
    fn should_return_an_error_when_there_are_no_commands_from_namespace() {
        let commands = build_commands();
        let invalid_namespace = String::from("invalid");
        let commands_from_namespace =
            commands.commands(invalid_namespace.clone(), String::from(""));

        if let Err(error) = commands_from_namespace {
            assert_eq!(
                format!(
                    "There are no commands to show for namespace \"{}\"",
                    invalid_namespace
                ),
                error.to_string()
            )
        }
    }

    #[test]
    fn should_remove_a_command() {
        let mut commands = build_commands();
        let all_commands = commands
            .commands(String::from("All"), String::from(""))
            .unwrap();

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
        let all_commands = commands
            .commands(String::from("All"), String::from(""))
            .unwrap();

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

        if let Ok(items) = commands.add_command(&new_command) {
            assert_eq!(3, items.len())
        }

        let mut edited_command = new_command.clone();
        edited_command.alias = String::from("edited_alias");

        let command_list_with_edited_command =
            commands.add_edited_command(&edited_command, &new_command);

        if let Ok(items) = command_list_with_edited_command {
            assert_eq!(3, items.len());
            assert!(items.contains(&edited_command));
            assert!(!items.contains(&new_command));
        }
    }

    #[test]
    fn should_return_an_error_when_edit_a_duplicated_alias_command() {
        let mut commands = build_commands();
        let current_command = Command::default();

        commands.add_command(&current_command).unwrap();

        assert_eq!(
            3,
            commands
                .commands(String::from("All"), String::from(""))
                .unwrap()
                .len()
        );

        let mut edited_command = current_command.clone();
        edited_command.description = Some(String::from("edited command"));
        edited_command.namespace = String::from("namespace1");

        let command_list_with_edited_command =
            commands.add_edited_command(&edited_command, &current_command);

        if let Err(error) = command_list_with_edited_command {
            assert_eq!(
                format!(
                    "Command with alias \"{}\" already exists in \"{}\" namespace",
                    edited_command.alias, edited_command.namespace
                ),
                error.to_string()
            )
        }
    }
}
