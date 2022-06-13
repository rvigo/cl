use std::char;

use crate::command_item::{CommandItem, CommandItemBuilder};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use log::info;
use tui::widgets::ListState;

#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    in_focus: bool,
    pub input: String,
}

impl Item {
    pub fn new(name: String, in_focus: bool) -> Item {
        Item {
            name,
            in_focus,
            input: String::from(""),
        }
    }

    pub fn get_mut_ref(&mut self) -> &mut Item {
        self
    }

    pub fn toggle_focus(&mut self) {
        self.in_focus = !self.in_focus
    }

    pub fn push(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop(&mut self) {
        self.input.pop();
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
    }
}

#[derive(Debug, Clone)]
pub struct OpsContext {
    pub focus_state: ListState,
    items: Vec<Item>,
    pub current_command: Option<CommandItem>,
}

impl OpsContext {
    pub fn new(items: Vec<(String, bool)>) -> OpsContext {
        let items = items
            .into_iter()
            .map(|(name, focus)| Item::new(name, focus))
            .collect_vec();
        OpsContext {
            items,
            focus_state: ListState::default(),
            current_command: None,
        }
    }
    pub fn get_vec_of_mut_items(&mut self) -> Vec<&mut Item> {
        self.items.iter_mut().collect_vec()
    }

    pub fn get_current_command(&self) -> Option<CommandItem> {
        self.current_command.clone()
    }

    pub fn set_current_command(&mut self, command: Option<CommandItem>) {
        self.current_command = command;
    }
    fn clear_inputs(&mut self) {
        for item in self.get_vec_of_mut_items() {
            item.clear_input()
        }
    }

    pub fn is_in_focus(&self, name: &str) -> bool {
        let item = self
            .items
            .get(self.focus_state.selected().unwrap())
            .unwrap();
        item.name == name && item.in_focus
    }

    pub fn next(&mut self) {
        let old_i = self.focus_state.selected().unwrap();
        self.items.get_mut(old_i).unwrap().toggle_focus();
        let i = match self.focus_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.focus_state.select(Some(i));
        self.items.get_mut(i).unwrap().toggle_focus();
    }

    pub fn previous(&mut self) {
        let old_i = self.focus_state.selected().unwrap();
        self.items.get_mut(old_i).unwrap().toggle_focus();
        let i = match self.focus_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.focus_state.select(Some(i));
        self.items.get_mut(i).unwrap().toggle_focus();
    }

    pub fn get_current_in_focus(&mut self) -> &mut Item {
        let current = self
            .items
            .get_mut(self.focus_state.selected().unwrap())
            .unwrap();
        current
    }

    pub fn get_component_input(&self, component_name: &str) -> String {
        self.items
            .clone()
            .iter_mut()
            .filter(|item| item.name == component_name)
            .next()
            .unwrap()
            .get_mut_ref()
            .input
            .clone()
    }

    pub fn build_command(&mut self) -> Result<CommandItem> {
        let mut command_builder = CommandItemBuilder::default();
        for item in self.items.clone().into_iter() {
            match item.name.as_str() {
                "alias" => {
                    command_builder.alias(item.input);
                }
                "command" => {
                    command_builder.command(item.input);
                }
                "description" => {
                    if item.input.is_empty() {
                        command_builder.description(None);
                    } else {
                        command_builder.description(Some(item.input));
                    }
                }
                "tags" => {
                    if item.input.is_empty() {
                        command_builder.tags(None);
                    } else {
                        command_builder.tags(Some(
                            item.input
                                .split(',')
                                .map(|char| char.to_string())
                                .collect_vec(),
                        ));
                    }
                }
                "namespace" => {
                    command_builder.namespace(item.input);
                }
                _ => {}
            }
        }

        let command = command_builder.build();
        match command.validate() {
            Ok(_) => {
                info!("cleaning inputs after validation if everything went ok");
                self.clear_inputs();
                Ok(command)
            }
            Err(error) => Err(anyhow!(error)),
        }
    }

    pub fn set_selected_command_inputs(&mut self) {
        let current_command = self.current_command.as_mut().unwrap();
        self.items
            .iter_mut()
            .for_each(|item| match item.name.as_str() {
                "alias" => {
                    info!("setting input alias to {}", current_command.alias);
                    item.get_mut_ref().input = current_command.alias.clone();
                }
                "command" => {
                    item.get_mut_ref().input = current_command.command.clone();
                }
                "description" => {
                    item.get_mut_ref().input = current_command
                        .description
                        .as_ref()
                        .unwrap_or(&String::from(""))
                        .to_string();
                }
                "tags" => {
                    item.get_mut_ref().input = current_command
                        .tags
                        .as_ref()
                        .unwrap_or(&vec![String::from("")])
                        .join(",")
                        .to_string();
                }
                "namespace" => item.get_mut_ref().input = current_command.namespace.clone(),
                _ => {}
            });
    }

    pub fn edit_command(&mut self) -> Result<CommandItem> {
        let mut command_item = self.get_current_command().unwrap();
        self.items.clone().iter().for_each(|item| {
            info!(
                "command item {} field will be set to {}",
                item.name, item.input
            );

            match item.name.as_str() {
                "alias" => {
                    command_item.alias = item.input.clone();
                }
                "command" => {
                    command_item.command = item.input.clone();
                }
                "description" => {
                    if item.input.is_empty() {
                        command_item.description = None;
                    } else {
                        command_item.description = Some(item.input.clone());
                    }
                }
                "tags" => {
                    if item.input.is_empty() {
                        command_item.tags = None;
                    } else {
                        command_item.tags = Some(
                            item.input
                                .split(',')
                                .map(|char| char.to_string())
                                .collect_vec(),
                        );
                    }
                }
                "namespace" => {
                    command_item.namespace = item.input.clone();
                }
                _ => {}
            }
            info!("command after edition: {:#?}", command_item);
        });
        match command_item.clone().validate() {
            Ok(_) => {
                info!("cleaning inputs after validation if everything went ok");
                self.clear_inputs();
            }
            Err(error) => return Err(anyhow!(error)),
        };
        Ok(command_item)
    }
}
