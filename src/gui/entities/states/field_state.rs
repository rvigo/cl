use crate::gui::screens::widgets::text_field::{FieldType, TextField};

use super::State;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct FieldState {
    selected: Option<FieldType>,
    original_fields: HashMap<FieldType, String>,
    edited_fields: HashMap<FieldType, String>,
}

impl FieldState {
    pub fn update_fields(&mut self, field: &TextField) {
        self.original_fields
            .insert(field.field_type.to_owned(), field.input_as_string());
        self.edited_fields
            .insert(field.field_type.to_owned(), field.input_as_string());
    }

    pub fn updated_edited_field(&mut self, field: &TextField) {
        let input = field.input_as_string();
        self.edited_fields
            .insert(field.field_type.to_owned(), input);
    }

    pub fn is_modified(&self) -> bool {
        for (field_type, original_value) in self.original_fields.iter() {
            if let Some(edited_value) = self.edited_fields.get(field_type) {
                if edited_value.ne(original_value) {
                    return true;
                }
            } else {
                return true;
            }
        }
        false
    }

    pub fn reset_fields_edition_state(&mut self) {
        let mut default_map = HashMap::new();
        for field_type in FieldType::iter() {
            default_map.insert(field_type.to_owned(), String::default());
        }

        self.original_fields = default_map.clone();
        self.edited_fields = default_map;
    }
}

impl State for FieldState {
    type Output = Option<FieldType>;

    fn selected(&self) -> Option<FieldType> {
        self.selected.to_owned()
    }

    fn select(&mut self, field_type: Option<FieldType>) {
        self.selected = field_type;
    }
}
