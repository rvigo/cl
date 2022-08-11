use super::field::{Field, FieldType};
use crate::command::{Command, CommandBuilder};
use anyhow::{bail, Result};
use itertools::Itertools;
use std::vec;
use tui::widgets::ListState;

pub struct Fields(Vec<Field>);

impl Default for Fields {
    fn default() -> Self {
        Fields(vec![
            Field::new(
                String::from("alias"),
                String::from(" Alias "),
                FieldType::Alias,
                true,
            ),
            Field::new(
                String::from("namespace"),
                String::from(" Namespace "),
                FieldType::Namespace,
                false,
            ),
            Field::new(
                String::from("command"),
                String::from(" Command "),
                FieldType::Command,
                false,
            ),
            Field::new(
                String::from("description"),
                String::from(" Description "),
                FieldType::Description,
                false,
            ),
            Field::new(
                String::from("tags"),
                String::from(" Tags "),
                FieldType::Tags,
                false,
            ),
        ])
    }
}

pub struct Context {
    pub focus_state: ListState,
    fields: Fields,
    pub current_command: Option<Command>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            fields: Fields::default(),
            focus_state: ListState::default(),
            current_command: None,
        }
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields.0
    }

    pub fn fields_mut(&mut self) -> &mut Vec<Field> {
        &mut self.fields.0
    }

    pub fn get_current_command(&self) -> Option<&Command> {
        self.current_command.as_ref()
    }

    pub fn set_current_command(&mut self, command: Option<Command>) {
        self.current_command = command;
    }

    pub fn clear_inputs(&mut self) {
        self.fields_mut()
            .iter_mut()
            .for_each(|field| field.clear_input());
    }

    pub fn is_in_focus(&self, name: &str) -> bool {
        let item = self
            .fields()
            .get(self.focus_state.selected().unwrap())
            .unwrap();
        item.name().eq(name) && item.in_focus()
    }

    pub fn next(&mut self) {
        let old_idx = self.focus_state.selected().unwrap();
        self.fields_mut().get_mut(old_idx).unwrap().toggle_focus();
        let idx = match self.focus_state.selected() {
            Some(i) => {
                if i >= self.fields.0.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.focus_state.select(Some(idx));
        self.fields.0.get_mut(idx).unwrap().toggle_focus();
    }

    pub fn previous(&mut self) {
        let old_idx = self.focus_state.selected().unwrap();
        self.fields_mut().get_mut(old_idx).unwrap().toggle_focus();
        let idx = match self.focus_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.fields.0.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.focus_state.select(Some(idx));
        self.fields_mut().get_mut(idx).unwrap().toggle_focus();
    }

    pub fn get_current_in_focus(&self) -> Option<&Field> {
        self.fields().get(self.focus_state.selected().unwrap())
    }

    pub fn get_current_in_focus_mut(&mut self) -> Option<&mut Field> {
        let idx = self.focus_state.selected().unwrap();
        self.fields_mut().get_mut(idx)
    }

    pub fn get_component_input(&self, component_name: &str) -> &str {
        self.fields()
            .iter()
            .find(|field| field.name().eq(component_name))
            .unwrap()
            .input
            .as_str()
    }

    pub fn build_command(&mut self) -> Result<Command> {
        let mut command_builder = CommandBuilder::default();
        self.fields_mut()
            .iter_mut()
            .for_each(|field| match field.field_type() {
                FieldType::Alias => {
                    command_builder.alias(field.input.to_string());
                }
                FieldType::Command => {
                    command_builder.command(field.input.to_string());
                }
                FieldType::Tags => {
                    if field.input.is_empty() {
                        command_builder.tags(None);
                    } else {
                        command_builder
                            .tags(Some(field.input.split(',').map(String::from).collect_vec()));
                    }
                }
                FieldType::Description => {
                    if field.input.is_empty() {
                        command_builder.description(None);
                    } else {
                        command_builder.description(Some(field.input.to_string()));
                    }
                }
                FieldType::Namespace => {
                    command_builder.namespace(field.input.to_string());
                }
                _ => {}
            });

        let command = command_builder.build();
        match command.validate() {
            Ok(_) => {
                self.clear_inputs();
                Ok(command)
            }
            Err(error) => bail!(error),
        }
    }

    pub fn set_selected_command_input(&mut self) {
        let current_command = self.current_command.as_mut().unwrap();
        self.fields.0.iter_mut().for_each(|field| {
            match field.field_type() {
                FieldType::Alias => field.input = current_command.alias.clone(),
                FieldType::Command => field.input = current_command.command.clone(),
                FieldType::Namespace => field.input = current_command.namespace.clone(),
                FieldType::Description => {
                    field.input = current_command
                        .description
                        .as_ref()
                        .unwrap_or(&String::from(""))
                        .to_string();
                }
                FieldType::Tags => {
                    field.input = current_command
                        .tags
                        .as_ref()
                        .unwrap_or(&vec![String::from("")])
                        .join(",")
                }
                _ => {}
            };
            field.reset_cursor_position();
        });
    }

    pub fn edit_command(&mut self) -> Result<Command> {
        let mut command = self
            .get_current_command()
            .map(|command| command.to_owned())
            .unwrap();
        self.fields()
            .iter()
            .for_each(|field| match field.field_type() {
                FieldType::Alias => command.alias = field.input.clone(),
                FieldType::Command => command.command = field.input.clone(),
                FieldType::Namespace => command.namespace = field.input.clone(),
                FieldType::Description => {
                    if field.input.is_empty() {
                        command.description = None;
                    } else {
                        command.description = Some(field.input.clone());
                    }
                }
                FieldType::Tags => {
                    if field.input.is_empty() {
                        command.tags = None;
                    } else {
                        command.tags = Some(field.input.split(',').map(String::from).collect_vec());
                    }
                }
                _ => {}
            });

        if let Err(error) = command.validate() {
            bail!(error)
        };
        self.clear_inputs();

        Ok(command)
    }
}
