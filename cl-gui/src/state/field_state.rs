use crate::{
    create_fields_map,
    terminal::TerminalSize,
    widget::{text_field::FieldType, TextField, WidgetKeyHandler},
};
use crossterm::event::KeyEvent;
use std::collections::HashMap;

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
pub struct FieldState<'a> {
    pub items: HashMap<FieldType, TextField<'a>>,
    sequence: Vec<FieldType>,
}

impl<'a> FieldState<'a> {
    pub fn new(size: &TerminalSize) -> Self {
        let items = create_fields_map! {
            FieldType::Alias: {
                focus = true,
                multiline = false
                },
            FieldType::Namespace: {
                focus = false,
                multiline = false
                },
            FieldType::Command: {
                focus = false,
                multiline = true
            },
            FieldType::Description: {
                focus = false,
                multiline = true
                },
            FieldType::Tags: {
                focus = false,
                multiline = false
                }
        };

        FieldState {
            items,
            sequence: match size {
                TerminalSize::Small => SMALL_SIZE_FIELD_SEQUENCE.to_owned(),
                TerminalSize::Medium | TerminalSize::Large => MEDIUM_SIZE_FIELD_SEQUENCE.to_owned(),
            },
        }
    }

    pub fn handle_input(&mut self, selected: &FieldType, input: KeyEvent) {
        if let Some(text_field) = self.items.get_mut(selected) {
            text_field.handle_input(input)
        }
    }

    pub fn get_field_mut(&mut self, field_type: &FieldType) -> Option<&mut TextField<'a>> {
        self.items.get_mut(field_type)
    }

    pub fn clear_inputs(&mut self) {
        self.items
            .iter_mut()
            .for_each(|(_, field)| field.clear_input());
    }
}

/// Iter related methods
impl<'field> FieldState<'field> {
    pub fn fields(&self) -> Vec<TextField<'field>> {
        let mut sorted_fields = vec![];

        self.sequence.iter().for_each(|i| {
            if let Some(f) = self.items.get(i) {
                sorted_fields.push(f.to_owned())
            }
        });

        sorted_fields.to_owned()
    }

    pub fn sort(&mut self, size: &TerminalSize) {
        let sequence = match size {
            TerminalSize::Small => SMALL_SIZE_FIELD_SEQUENCE.to_owned(),
            TerminalSize::Medium | TerminalSize::Large => MEDIUM_SIZE_FIELD_SEQUENCE.to_owned(),
        };

        self.sequence = sequence
    }

    pub fn sequence(&self) -> Vec<FieldType> {
        self.sequence.to_owned()
    }
}

impl<'a> From<(HashMap<FieldType, TextField<'a>>, Vec<FieldType>)> for FieldState<'a> {
    fn from((items, sequence): (HashMap<FieldType, TextField<'a>>, Vec<FieldType>)) -> Self {
        FieldState { items, sequence }
    }
}
