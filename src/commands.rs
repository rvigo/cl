use crate::{command_item::CommandItem, file_service};
use anyhow::{anyhow, Result};

use itertools::Itertools;
use log::info;
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

    pub fn add_command(&mut self, command_item: CommandItem) -> Result<()> {
        if self.clone().command_already_exists(&command_item) {
            return Err(anyhow!(
                "Command with alias \"{}\" already exists in \"{}\" namespace",
                command_item.alias,
                command_item.namespace
            ));
        }

        self.items.push(command_item);
        self.save_items()?;
        Ok(())
    }

    fn command_already_exists(&mut self, command_item: &CommandItem) -> bool {
        self.items
            .clone()
            .into_iter()
            .any(|c| c.alias == command_item.alias && c.namespace.eq(&command_item.namespace))
    }

    pub fn remove(&mut self, alias: String, namespace: Option<String>) -> Result<String> {
        let commands = self
            .items
            .clone()
            .into_iter()
            .filter(|command| {
                if namespace.is_some() {
                    command.alias.eq(&alias) && command.namespace.eq(namespace.as_ref().unwrap())
                } else {
                    command.alias.eq(&alias)
                }
            })
            .collect::<Vec<CommandItem>>();
        if commands.is_empty() {
            return Err(anyhow!("Cannot find command with alias {alias}"));
        } else if commands.len() > 1 {
            return Err(anyhow!(
                "There are multiples commands with alias \"{alias}\" in differents namespaces:\n{}\n\
            Please try again with the [namespace] flag",
                commands
                    .into_iter()
                    .map(|c| format!(" - alias '{}' in namespace {}", alias, c.namespace))
                    .collect::<Vec<String>>()
                    .join("\n")
            ));
        }
        self.items.retain(|command| {
            if namespace.is_some() {
                !command.alias.eq(&alias) || !namespace.as_ref().unwrap().eq(&command.namespace)
            } else {
                !command.alias.eq(&alias)
            }
        });
        self.save_items()?;
        Ok(String::from("Alias removed"))
    }

    pub fn all_commands(&self) -> Vec<CommandItem> {
        self.items.clone()
    }

    pub fn commands_from_namespace(&self, namespace: String) -> Result<Vec<CommandItem>> {
        info!("getting commands from {namespace}");
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

    pub fn exec_alias(
        &mut self,
        alias: String,
        mut args: Vec<String>,
        namespace: Option<String>,
    ) -> Result<String> {
        Ok(String::from("ok"))
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
    }

    pub fn commands_by_tag(&self, tag: String) -> Result<Vec<CommandItem>> {
        let commands: Vec<CommandItem> = self
            .items
            .clone()
            .into_iter()
            .filter(|c| c.tags.is_some() && c.tags.as_ref().unwrap().contains(&tag))
            .collect();

        if commands.is_empty() {
            Err(anyhow!("No commands found for tag \"{tag}\""))
        } else {
            Ok(commands)
        }
    }

    // pub fn info(&self) {
    //     let commands = self.clone().items.into_iter().collect::<Vec<_>>().len();
    //     let tags = self
    //         .clone()
    //         .items
    //         .into_iter()
    //         .filter(|item| item.tags.is_some())
    //         .flat_map(|item| item.tags.unwrap())
    //         .collect::<HashSet<String>>()
    //         .len();
    //     let namespaces = self.clone().namespaces().len();
    //     let aliases = self
    //         .clone()
    //         .items
    //         .into_iter()
    //         .filter(|item| item.alias.is_some())
    //         .map(|item| item.alias.unwrap())
    //         .collect::<Vec<String>>()
    //         .len();
    //     println!("commands: {commands}\nnamespaces: {namespaces}\naliases: {aliases}\ntags: {tags}")
    // }

    fn save_items(&self) -> Result<()> {
        file_service::write_to_file(self.items.clone())
    }

    pub fn get_command_item_ref(&self, idx: usize) -> Option<&CommandItem> {
        self.items.get(idx)
    }

    pub fn get_items_mut_ref(&mut self) -> &mut Vec<CommandItem> {
        &mut self.items
    }
}

impl Iterator for Commands {
    type Item = CommandItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_items_mut_ref().clone().into_iter().next()
    }
}
