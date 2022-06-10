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

    pub fn toggle_focus(&mut self) {
        self.in_focus = !self.in_focus
    }
}

#[derive(Debug, Clone)]
pub struct InsertContext {
    pub focus_state: ListState,
    items: Vec<Item>,
}

impl InsertContext {
    pub fn new(items: Vec<(String, bool)>) -> InsertContext {
        let items = items
            .into_iter()
            .map(|(name, focus)| Item::new(name, focus))
            .collect_vec();
        InsertContext {
            items,
            focus_state: ListState::default(),
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
            .into_iter()
            .filter(|item| item.name == component_name)
            .next()
            .unwrap()
            .input
    }

    pub fn build_command(&mut self) -> Result<CommandItem> {
        let mut command_builder = CommandItemBuilder::default();
        for item in self.items.clone().into_iter() {
            match item.name.as_str() {
                "alias" => {
                    if item.input.is_empty() {
                        command_builder.alias(item.input);
                    }
                }
                "command" => {
                    command_builder.command(item.input);
                }
                "description" => {
                    if item.input.is_empty() {
                        command_builder.description(Some(item.input));
                    } else {
                        command_builder.description(None);
                    }
                }
                "tags" => {
                    if item.input.is_empty() {
                        command_builder.tags(Some(
                            item.input
                                .split(',')
                                .map(|char| char.to_string())
                                .collect_vec(),
                        ));
                    } else {
                        command_builder.tags(None);
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
                self.clear_inputs();
                info!("new command created: {:?}", command);
                Ok(command)
            }
            Err(error) => Err(anyhow!(error)),
        }
    }

    pub fn get_items_mut_ref(&mut self) -> &mut Vec<Item> {
        &mut self.items
    }

    fn clear_inputs(&mut self) {
        for mut item in self.get_items_mut_ref().into_iter() {
            item.input = String::from("")
        }
    }
}
