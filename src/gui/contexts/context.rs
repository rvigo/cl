use std::char;

use crate::command_item::{CommandItem, CommandItemBuilder};
use anyhow::{bail, Result};
use itertools::Itertools;
use tui::widgets::ListState;

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

pub struct Context {
    pub focus_state: ListState,
    items: Vec<Item>,
    pub current_command: Option<CommandItem>,
}

impl Context {
    pub fn new(items: Vec<(String, bool)>) -> Context {
        let items = items
            .into_iter()
            .map(|(name, focus)| Item::new(name, focus))
            .collect_vec();
        Context {
            items,
            focus_state: ListState::default(),
            current_command: None,
        }
    }

    pub fn get_current_command(&self) -> Option<&CommandItem> {
        self.current_command.as_ref()
    }

    pub fn set_current_command(&mut self, command: Option<CommandItem>) {
        self.current_command = command;
    }

    fn clear_inputs(&mut self) {
        for item in self.items.iter_mut().collect_vec() {
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

    pub fn get_component_input(&mut self, component_name: &str) -> String {
        self.items
            .iter_mut()
            .find(|item| item.name == component_name)
            .unwrap()
            .input
            .clone()
    }

    pub fn build_command(&mut self) -> Result<CommandItem> {
        let mut command_builder = CommandItemBuilder::default();
        for item in self.items.iter() {
            match item.name.as_str() {
                "alias" => {
                    command_builder.alias(item.input.to_string());
                }
                "command" => {
                    command_builder.command(item.input.to_string());
                }
                "description" => {
                    if item.input.is_empty() {
                        command_builder.description(None);
                    } else {
                        command_builder.description(Some(item.input.to_string()));
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
                    command_builder.namespace(item.input.to_string());
                }
                _ => {}
            }
        }

        let command = command_builder.build();
        match command.validate() {
            Ok(_) => {
                self.clear_inputs();
                Ok(command)
            }
            Err(error) => bail!(error),
        }
    }

    pub fn set_selected_command_inputs(&mut self) {
        let current_command = self.current_command.as_mut().unwrap();
        self.items
            .iter_mut()
            .for_each(|item| match item.name.as_str() {
                "alias" => item.input = current_command.alias.clone(),
                "command" => item.input = current_command.command.clone(),
                "namespace" => item.input = current_command.namespace.clone(),
                "description" => {
                    item.input = current_command
                        .description
                        .as_ref()
                        .unwrap_or(&String::from(""))
                        .to_string();
                }
                "tags" => {
                    item.input = current_command
                        .tags
                        .as_ref()
                        .unwrap_or(&vec![String::from("")])
                        .join(",")
                }

                _ => {}
            });
    }

    pub fn edit_command(&mut self) -> Result<CommandItem> {
        let mut command_item = self
            .get_current_command()
            .map(|item| item.to_owned())
            .unwrap();
        self.items.iter().for_each(|item| match item.name.as_str() {
            "alias" => command_item.alias = item.input.clone(),
            "command" => command_item.command = item.input.clone(),
            "namespace" => command_item.namespace = item.input.clone(),
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

            _ => {}
        });

        if let Err(error) = command_item.validate() {
            bail!(error)
        };
        self.clear_inputs();

        Ok(command_item)
    }
}
