use crate::command_item::CommandItem;
use anyhow::{bail, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Commands {
    items: Vec<CommandItem>,
}

impl Commands {
    pub fn init(items: Vec<CommandItem>) -> Commands {
        return Self { items };
    }

    pub fn get_ref(&self) -> &Commands {
        self
    }

    pub fn get_command_item_ref(&self, idx: usize) -> Option<&CommandItem> {
        self.items.get(idx)
    }

    pub fn namespaces(&mut self) -> Vec<String> {
        let mut keys: Vec<String> = self
            .items
            .clone()
            .into_iter()
            .map(|command| command.namespace)
            .unique()
            .collect();
        keys.sort();
        keys
    }

    pub fn add_command(&mut self, command_item: &CommandItem) -> Result<Vec<CommandItem>> {
        if self.clone().command_already_exists(&command_item) {
            bail!(
                "Command with alias \"{}\" already exists in \"{}\" namespace",
                command_item.alias,
                command_item.namespace
            );
        }

        self.items.push(command_item.clone());
        Ok(self.items.clone())
    }

    pub fn add_edited_command(
        &mut self,
        edited_command_item: &CommandItem,
        current_command_item: &CommandItem,
    ) -> Result<Vec<CommandItem>> {
        if self.items.clone().into_iter().any(|c| {
            c.alias.eq(&edited_command_item.alias)
                && !edited_command_item.alias.eq(&current_command_item.alias)
                && c.namespace.eq(&edited_command_item.namespace)
        }) {
            bail!(
                "Command with alias \"{}\" already exists in \"{}\" namespace",
                edited_command_item.alias,
                edited_command_item.namespace
            );
        }
        self.items.retain(|command| {
            !command.alias.eq(&current_command_item.alias)
                && !command.namespace.eq(&current_command_item.namespace)
        });

        self.items.push(edited_command_item.clone());
        Ok(self.items.clone())
    }

    fn command_already_exists(&mut self, command_item: &CommandItem) -> bool {
        self.items
            .clone()
            .into_iter()
            .any(|c| c.alias == command_item.alias && c.namespace.eq(&command_item.namespace))
    }

    pub fn remove(&mut self, command_item: &CommandItem) -> Result<Vec<CommandItem>> {
        self.items.retain(|command| {
            !command.alias.eq(&command_item.alias) || !command_item.namespace.eq(&command.namespace)
        });
        Ok(self.items.clone())
    }

    pub fn all_commands(&mut self) -> Vec<CommandItem> {
        if self.items.is_empty() {
            vec![CommandItem::default()]
        } else {
            self.items.clone()
        }
    }

    pub fn commands_from_namespace(&mut self, namespace: String) -> Result<Vec<CommandItem>> {
        let commands: Vec<CommandItem> = self
            .items
            .clone()
            .into_iter()
            .filter(|command| command.namespace == namespace)
            .collect();

        if commands.is_empty() {
            bail!("There are no commands to show for namespace \"{namespace}\".");
        }

        Ok(commands)
    }

    pub fn exec_command(&self, command_item: &CommandItem) -> Result<()> {
        let shell = env::var("SHELL").unwrap_or(String::from("sh"));
        println!("{} {}", shell, command_item.command);
        let output = std::process::Command::new(shell)
            .arg("-c")
            .arg(command_item.command.to_string())
            .spawn();

        output?.wait()?;
        Ok(())
    }

    pub fn find_command(&self, alias: String, namespace: Option<String>) -> Result<CommandItem> {
        let commands: Vec<CommandItem> = self
            .items
            .clone()
            .into_iter()
            .filter(|c| {
                if namespace.is_none() {
                    true
                } else {
                    c.namespace.eq(namespace.as_ref().unwrap())
                }
            })
            .filter(|c| c.alias.eq(&alias))
            .collect();

        if commands.len() > 1 {
            bail!(
                "There are commands with the alias \'{}\' in multiples namespaces. \
            Please use the \'--namespace\' flag",
                alias
            )
        } else if commands.is_empty() {
            bail!("The command \'{}\' was not found!", alias)
        } else {
            Ok(commands.first().unwrap().to_owned())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::command_item::CommandItemBuilder;

    fn build_commands() -> Commands {
        let mut command1 = CommandItemBuilder::default();
        command1
            .alias(String::from("test alias"))
            .namespace(String::from("test namespace1"))
            .command(String::from("test command"));
        let mut command2 = CommandItemBuilder::default();
        command2
            .alias(String::from("test alias"))
            .namespace(String::from("test namespace2"))
            .command(String::from("test command"));

        let commands = Commands::init(vec![command1.build(), command2.build()]);
        commands
    }

    #[test]
    fn should_return_all_namespaces() {
        let mut commands = build_commands();
        let namespaces = commands.namespaces();
        assert_eq!(
            vec![
                String::from("test namespace1"),
                String::from("test namespace2")
            ],
            namespaces
        )
    }

    #[test]
    fn should_return_all_commands() {
        let mut commands = build_commands();
        let all_command_items = commands.all_commands();
        assert_eq!(2, all_command_items.len())
    }

    #[test]
    fn should_return_welcome_command_when_there_is_no_saved_command() {
        let mut commands = Commands::init(Vec::default());
        let all_command_items = commands.all_commands();
        let default_command_item = CommandItem::default();
        assert_eq!(all_command_items.len(), 1);
        assert_eq!(
            default_command_item,
            all_command_items.get(0).unwrap().to_owned()
        );
    }

    #[test]
    fn should_validate_if_command_already_exists() {
        let mut commands = build_commands();
        let mut duplicated_command = CommandItemBuilder::default();
        duplicated_command
            .alias(String::from("test alias"))
            .namespace(String::from("test namespace1"))
            .command(String::from("test command"));

        let already_exists = commands.command_already_exists(&duplicated_command.build());
        assert_eq!(true, already_exists)
    }

    #[test]
    fn should_return_all_commands_from_namespace() {
        let mut commands = build_commands();
        let commands_from_namespace =
            commands.commands_from_namespace(String::from("test namespace2"));

        if let Ok(items) = commands_from_namespace {
            assert_eq!(1, items.len())
        }
    }

    #[test]
    fn should_return_an_error_when_there_are_no_commands_from_namespace() {
        let mut commands = build_commands();
        let invalid_namespace = String::from("invalid");
        let commands_from_namespace = commands.commands_from_namespace(invalid_namespace.clone());

        if let Err(error) = commands_from_namespace {
            assert_eq!(
                format!(
                    "There are no commands to show for namespace \"{}\".",
                    invalid_namespace
                ),
                error.to_string()
            )
        }
    }

    #[test]
    fn should_remove_a_command() {
        let mut commands = build_commands();
        let all_commands = commands.all_commands();

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
        let all_commands = commands.all_commands();

        assert_eq!(2, all_commands.len());

        let new_command = CommandItem::default();
        let new_command_list = commands.add_command(&new_command);

        if let Ok(items) = new_command_list {
            assert_eq!(3, items.len());
            assert!(items.contains(&new_command))
        }
    }

    #[test]
    fn should_add_an_edited_command() -> Result<()> {
        let mut commands = build_commands();
        let current_command = CommandItem::default();

        commands.add_command(&current_command)?;

        assert_eq!(3, commands.all_commands().len());

        let mut edited_command = current_command.clone();
        edited_command.description = Some(String::from("edited command"));

        let command_list_with_edited_command =
            commands.add_edited_command(&edited_command, &current_command);

        if let Ok(items) = command_list_with_edited_command {
            assert_eq!(3, items.len());
            assert!(items.contains(&edited_command));
            assert!(!items.contains(&current_command));
        }

        Ok(())
    }

    #[test]
    fn should_return_an_error_when_edit_a_duplicated_alias_command() -> Result<()> {
        let mut commands = build_commands();
        let current_command = CommandItem::default();

        commands.add_command(&current_command)?;

        assert_eq!(3, commands.all_commands().len());

        let mut edited_command = current_command.clone();
        edited_command.description = Some(String::from("edited command"));
        edited_command.namespace = String::from("test namespace1");

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
        Ok(())
    }
}
