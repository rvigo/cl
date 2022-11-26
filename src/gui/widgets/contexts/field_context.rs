use crate::{
    command::{Command, CommandBuilder},
    gui::widgets::{
        field::{Field, FieldType},
        fields::Fields,
    },
};
use anyhow::{bail, Result};
use itertools::Itertools;
use tui::widgets::ListState;
use tui_textarea::TextArea;

#[derive(Default)]
pub struct FieldContext<'a> {
    pub fields: Fields<'a>,
    pub focus_state: ListState,
    selected_command: Option<Command>,
}

impl<'a> FieldContext<'a> {
    pub fn next_field(&mut self) {
        let old_idx = self.focus_state.selected().unwrap();
        self.fields.get_mut(old_idx).unwrap().toggle_focus();
        let idx = match self.focus_state.selected() {
            Some(i) => {
                if i >= self.fields.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.focus_state.select(Some(idx));
        self.fields.get_mut(idx).unwrap().toggle_focus();
    }

    pub fn previous_field(&mut self) {
        let old_idx = self.focus_state.selected().unwrap();
        self.fields.get_mut(old_idx).unwrap().toggle_focus();
        let idx = match self.focus_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.fields.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.focus_state.select(Some(idx));
        self.fields.get_mut(idx).unwrap().toggle_focus();
    }

    pub fn selected_mut_field(&mut self) -> Option<&mut Field<'a>> {
        let idx = self.focus_state.selected().unwrap_or(0);
        self.fields.get_mut(idx)
    }

    pub fn clear_fields_input(&mut self) {
        self.fields.iter_mut().for_each(|field| field.clear_input());
    }

    pub fn build_new_command(&mut self) -> Result<Command> {
        let mut command_builder = CommandBuilder::default();
        self.fields
            .iter_mut()
            .for_each(|field| match field.field_type {
                FieldType::Alias => {
                    command_builder.alias(field.input_as_string());
                }
                FieldType::Command => {
                    command_builder.command(field.input_as_string());
                }
                FieldType::Tags => {
                    if field.text_area.is_empty() {
                        command_builder.tags(None);
                    } else {
                        command_builder.tags(Some(
                            field
                                .input_as_string()
                                .split(',')
                                .map(|tag| String::from(tag.trim()))
                                .filter(|tag| !tag.is_empty())
                                .collect_vec(),
                        ));
                    }
                }
                FieldType::Description => {
                    if field.text_area.is_empty() {
                        command_builder.description(None);
                    } else {
                        command_builder.description(Some(field.input_as_string()));
                    }
                }
                FieldType::Namespace => {
                    command_builder.namespace(field.input_as_string());
                }
            });

        let command = command_builder.build();
        match command.validate() {
            Ok(_) => Ok(command),
            Err(error) => bail!(error),
        }
    }

    pub fn edit_command(&mut self) -> Result<Command> {
        let mut command = self
            .selected_command()
            .map(|command| command.to_owned())
            .unwrap();
        self.fields
            .iter_mut()
            .for_each(|field| match field.field_type {
                FieldType::Alias => command.alias = field.input_as_string(),
                FieldType::Command => command.command = field.input_as_string(),
                FieldType::Namespace => command.namespace = field.input_as_string(),
                FieldType::Description => {
                    if field.text_area.is_empty() {
                        command.description = None;
                    } else {
                        command.description = Some(field.input_as_string());
                    }
                }
                FieldType::Tags => {
                    if field.text_area.is_empty() {
                        command.tags = None;
                    } else {
                        command.tags = Some(
                            field
                                .input_as_string()
                                .split(',')
                                .map(|tag| String::from(tag.trim()))
                                .filter(|tag| !tag.is_empty())
                                .collect_vec(),
                        );
                    }
                }
            });

        command.validate()?;
        Ok(command)
    }

    pub fn selected_command(&self) -> Option<&Command> {
        self.selected_command.as_ref()
    }

    pub fn select_command(&mut self, selected_command: Option<Command>) {
        self.selected_command = selected_command
    }

    pub fn reset_fields(&mut self) {
        self.fields = Fields::build_form_fields()
    }

    pub fn set_selected_command_input(&mut self) {
        let current_command = self.selected_command.as_mut().unwrap();
        self.fields.iter_mut().for_each(|field| {
            match field.field_type {
                FieldType::Alias => {
                    field.text_area = TextArea::from(vec![current_command.alias.clone()])
                }
                FieldType::Command => {
                    field.text_area = current_command
                        .command
                        .clone()
                        .lines()
                        .map(String::from)
                        .collect()
                }

                FieldType::Namespace => {
                    field.text_area = TextArea::from(vec![current_command.namespace.clone()])
                }
                FieldType::Description => {
                    field.text_area = TextArea::from(
                        current_command
                            .description
                            .as_ref()
                            .unwrap_or(&String::from(""))
                            .clone()
                            .lines()
                            .map(String::from)
                            .collect::<Vec<String>>(),
                    );
                }
                FieldType::Tags => {
                    field.text_area = TextArea::from(vec![current_command
                        .tags
                        .as_ref()
                        .unwrap_or(&vec![String::from("")])
                        .join(", ")])
                }
            };
        });
    }
}
