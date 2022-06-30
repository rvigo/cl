use crate::{command_item::CommandItem, file_service};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Commands {
    items: Vec<CommandItem>,
}

impl Commands {
    pub fn init() -> Commands {
        return Self {
            items: file_service::load_commands_file(),
        };
    }

    pub fn get_ref(&self) -> &Commands {
        self
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

    pub fn add_command(&mut self, command_item: &CommandItem) -> Result<()> {
        if self.clone().command_already_exists(&command_item) {
            return Err(anyhow!(
                "Command with alias \"{}\" already exists in \"{}\" namespace",
                command_item.alias,
                command_item.namespace
            ));
        }

        self.items.push(command_item.clone());
        self.save_items()?;
        Ok(())
    }

    pub fn add_edited_command(
        &mut self,
        edited_command_item: &CommandItem,
        current_command_item: &CommandItem,
    ) -> Result<()> {
        if self.items.clone().into_iter().any(|c| {
            c.alias.eq(&edited_command_item.alias)
                && !edited_command_item.alias.eq(&current_command_item.alias)
                && c.namespace.eq(&edited_command_item.namespace)
        }) {
            return Err(anyhow!(
                "Command with alias \"{}\" already exists in \"{}\" namespace",
                edited_command_item.alias,
                edited_command_item.namespace
            ));
        }
        self.items.clone().retain(|command| {
            !command.alias.eq(&current_command_item.alias)
                && !command.namespace.eq(&current_command_item.namespace)
        });
        self.items.push(edited_command_item.clone());
        self.save_items()?;
        Ok(())
    }

    fn command_already_exists(&mut self, command_item: &CommandItem) -> bool {
        self.items
            .clone()
            .into_iter()
            .any(|c| c.alias == command_item.alias && c.namespace.eq(&command_item.namespace))
    }

    pub fn remove(&mut self, command_item: &CommandItem) -> Result<String> {
        self.items.retain(|command| {
            !command.alias.eq(&command_item.alias) || !command_item.namespace.eq(&command.namespace)
        });
        self.save_items()?;
        Ok(String::from("Alias removed"))
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
            return Err(anyhow!(
                "There are no commands to show for namespace \"{namespace}\"."
            ));
        }
        Ok(commands)
    }

    pub fn exec_command(&self, command_item: &CommandItem) -> Result<()> {
        let shell = env::var("SHELL").unwrap_or(String::from("sh"));
        let output = std::process::Command::new(shell)
            .arg("-c")
            .arg(command_item.command.to_string())
            .spawn();

        output?.wait()?;
        Ok(())
    }

    fn save_items(&mut self) -> Result<()> {
        file_service::write_to_file(self.items.clone())
    }

    pub fn get_command_item_ref(&self, idx: usize) -> Option<&CommandItem> {
        self.items.get(idx)
    }
}
