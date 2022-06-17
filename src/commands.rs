use crate::{command_item::CommandItem, file_service};
use anyhow::{anyhow, Result};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

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
        self.items.retain(|command| {
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

    pub fn all_commands(&self) -> Vec<CommandItem> {
        self.items.clone()
    }

    pub fn commands_from_namespace(&self, namespace: String) -> Result<Vec<CommandItem>> {
        let mut commands: Vec<CommandItem> = self
            .items
            .clone()
            .into_iter()
            .filter(|command| command.namespace == namespace)
            .collect();
        commands.sort();
        if commands.is_empty() {
            return Err(anyhow!(
                "There are no commands to show for namespace \"{namespace}\"."
            ));
        }
        Ok(commands)
    }

    // pub fn exec_alias(
    //     &mut self,
    //     alias: String,
    //     mut args: Vec<String>,
    //     namespace: Option<String>,
    // ) -> Result<String> {
    //     Ok(String::from("ok"))
    // match self.command_from_alias(&alias, namespace) {
    //     Ok(command) => {
    //         args.insert(0, command);

    //         let shell = env::var("SHELL").unwrap_or(String::from("sh"));
    //         println!(" - running {} {}", shell, &args.clone().join(" "));
    //         let output = std::process::Command::new(shell)
    //             .arg("-c")
    //             .arg(args.join(" "))
    //             .output();

    //         match output {
    //             Ok(value) => {
    //                 if value.status.success() {
    //                     Ok(String::from_utf8_lossy(&value.stdout).to_string())
    //                 } else {
    //                     Err(anyhow!(String::from_utf8_lossy(&value.stderr).to_string()))
    //                 }
    //             }
    //             Err(error) => Err(anyhow!(error.to_string())),
    //         }
    //     }
    //     Err(error) => Err(anyhow!(error.to_string())),
    // }
    // }

    fn save_items(&mut self) -> Result<()> {
        self.items.sort();
        file_service::write_to_file(self.items.clone())
    }

    pub fn get_command_item_ref(&self, idx: usize) -> Option<&CommandItem> {
        self.items.get(idx)
    }
}
