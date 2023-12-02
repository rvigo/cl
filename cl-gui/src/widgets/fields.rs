use crate::entities::terminal::TerminalSize;

use super::text_field::{FieldType, TextField, TextFieldBuilder};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

const SMALL_SIZE_FIELD_SEQUENCE: &[FieldType] = &[
    FieldType::Alias,
    FieldType::Namespace,
    FieldType::Description,
    FieldType::Tags,
    FieldType::Command,
];

const MEDIUM_SIZE_FIELD_SEQUENCE: &[FieldType] = &[
    FieldType::Alias,
    FieldType::Namespace,
    FieldType::Command,
    FieldType::Description,
    FieldType::Tags,
];

#[derive(Clone)]
pub struct Fields<'a> {
    items: HashMap<FieldType, TextField<'a>>,
    sequence: Vec<FieldType>,
}

impl<'a> Fields<'a> {
    pub fn new(size: &TerminalSize) -> Self {
        let alias = TextFieldBuilder::default()
            .field_type(FieldType::Alias)
            .in_focus(true)
            .multiline(false)
            .build();
        let namespace = TextFieldBuilder::default()
            .field_type(FieldType::Namespace)
            .in_focus(false)
            .multiline(false)
            .build();
        let command = TextFieldBuilder::default()
            .field_type(FieldType::Command)
            .in_focus(false)
            .multiline(true)
            .build();
        let description = TextFieldBuilder::default()
            .field_type(FieldType::Description)
            .in_focus(false)
            .multiline(true)
            .build();
        let tags = TextFieldBuilder::default()
            .field_type(FieldType::Tags)
            .in_focus(false)
            .multiline(false)
            .build();

        let map = vec![alias, namespace, command, description, tags]
            .into_iter()
            .map(|f| (f.field_type(), f))
            .collect();

        Fields {
            items: map,
            sequence: match size {
                TerminalSize::Small => SMALL_SIZE_FIELD_SEQUENCE.to_owned(),
                TerminalSize::Medium | TerminalSize::Large => MEDIUM_SIZE_FIELD_SEQUENCE.to_owned(),
            },
        }
    }

    pub fn get_fields_iter(&self) -> impl Iterator<Item = TextField<'a>> {
        let mut sorted_fields = vec![];

        self.sequence.iter().for_each(|i| {
            if let Some(f) = self.items.get(i) {
                sorted_fields.push(f.to_owned())
            }
        });

        sorted_fields.into_iter()
    }

    pub fn sort_by_terminal_size(&mut self, size: &TerminalSize) {
        let sequence = match size {
            TerminalSize::Small => SMALL_SIZE_FIELD_SEQUENCE.to_owned(),
            TerminalSize::Medium | TerminalSize::Large => MEDIUM_SIZE_FIELD_SEQUENCE.to_owned(),
        };

        self.sequence = sequence
    }

    pub fn get_field_mut(&mut self, field_type: &FieldType) -> Option<&mut TextField<'a>> {
        self.items.get_mut(field_type)
    }

    pub fn get_sequence(&self) -> Vec<FieldType> {
        self.sequence.to_owned()
    }

    pub fn clear_inputs(&mut self) {
        self.items
            .iter_mut()
            .for_each(|(_, field)| field.clear_input());
    }
}

impl<'a> From<(HashMap<FieldType, TextField<'a>>, Vec<FieldType>)> for Fields<'a> {
    fn from((items, sequence): (HashMap<FieldType, TextField<'a>>, Vec<FieldType>)) -> Self {
        Fields { items, sequence }
    }
}

impl<'a> Deref for Fields<'a> {
    type Target = HashMap<FieldType, TextField<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl DerefMut for Fields<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}
