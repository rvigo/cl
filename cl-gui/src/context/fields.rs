use super::{FieldMap, Selectable};
use crate::{
    state::{FieldState, State},
    terminal::TerminalSize,
    widget::text_field::{FieldType, TextField},
};
use cl_core::{hashmap, Command};
use crossterm::event::KeyEvent;
use std::collections::HashMap;

pub struct Fields<'fields> {
    state: FieldState<'fields>,
    selected_field: Option<FieldType>,
    original_fields: HashMap<FieldType, String>,
    edited_fields: HashMap<FieldType, String>,
}

impl<'fields> Fields<'fields> {
    pub fn new(size: &TerminalSize) -> Self {
        Self {
            state: FieldState::new(size),
            selected_field: None,
            original_fields: hashmap!(),
            edited_fields: hashmap!(),
        }
    }

    pub fn sort(&mut self, size: &TerminalSize) {
        self.state.sort(size);
    }

    pub fn inner(&self) -> Vec<TextField<'_>> {
        self.state.fields()
    }

    pub fn selected_field_mut(&mut self) -> Option<&mut TextField<'fields>> {
        self.selected()
            .and_then(|field| self.state.items.get_mut(&field))
    }

    pub fn collect(&self) -> FieldMap {
        let fields = self
            .state
            .items
            .iter()
            .map(|(t, v)| (t.to_owned(), v.text()))
            .collect();

        FieldMap(fields)
    }

    pub fn popuplate(&mut self, current_command: &Command) {
        self.state.items.iter_mut().for_each(|(field_type, field)| {
            let text = match field_type {
                FieldType::Alias => current_command.alias.to_owned(),
                FieldType::Command => current_command.command.to_owned(),
                FieldType::Namespace => current_command.namespace.to_owned(),
                FieldType::Description => current_command.description.clone().unwrap_or_default(),
                FieldType::Tags => current_command.tags.as_ref().unwrap_or(&vec![]).join(", "),
                _ => panic!("Invalid field type: {:?}", field_type),
            };
            field.set_text(text.to_string());
            field.move_cursor_to_end_of_text();

            self.original_fields
                .insert(field.field_type(), field.text());

            self.edited_fields.insert(field.field_type(), field.text());
        });
    }

    pub fn handle_input(&mut self, input: KeyEvent) {
        if let Some(selected) = self.selected() {
            self.state.handle_input(&selected, input);

            self.state
                .items
                .get(&selected)
                .map(|field| self.edited_fields.insert(selected, field.text()));
        }
    }

    pub fn clear_inputs(&mut self) {
        let mut default_map = hashmap!();

        for field_type in FieldType::values() {
            default_map.insert(field_type.to_owned(), String::default());
        }

        self.original_fields = default_map.to_owned();
        self.edited_fields = default_map;

        self.state.clear_inputs()
    }

    pub fn reset(&mut self) {
        self.selected_field_mut().map(TextField::deactivate_focus);
        self.select(Some(FieldType::default()));
        self.selected_field_mut().map(TextField::activate_focus);
    }

    pub fn is_modified(&self) -> bool {
        self.original_fields
            .iter()
            .any(|(field_type, original_value)| {
                if let Some(edited_value) = self.edited_fields.get(field_type) {
                    edited_value.ne(original_value)
                } else {
                    true
                }
            })
    }
}

impl Selectable for Fields<'_> {
    fn next(&mut self) {
        if let Some(current_field_type) = self.selected() {
            self.state
                .get_field_mut(&current_field_type)
                .map(TextField::deactivate_focus);

            let sequence = self.state.sequence();

            if let Some(pos) = sequence.iter().position(|x| current_field_type.eq(x)) {
                let new_field_idx = (pos + 1) % sequence.len();
                let new_field_type = self.state.sequence()[new_field_idx].to_owned();

                // selects the new field type
                self.select(Some(new_field_type));

                self.selected()
                    .and_then(|selected| self.state.get_field_mut(&selected))
                    .map(TextField::activate_focus);
            };
        }
    }

    fn previous(&mut self) {
        if let Some(current_field_type) = self.selected() {
            if let Some(field) = self.state.get_field_mut(&current_field_type) {
                field.deactivate_focus()
            }

            let order = self.state.sequence();
            if let Some(pos) = order.iter().position(|x| current_field_type.eq(x)) {
                let new_field_idx = (pos + order.len() - 1) % order.len();
                let new_field_type = self.state.sequence()[new_field_idx].to_owned();

                // selects the new field type
                self.select(Some(new_field_type));

                self.selected()
                    .and_then(|selected| self.state.get_field_mut(&selected))
                    .map(TextField::activate_focus);
            }
        }
    }
}

impl State for Fields<'_> {
    type Output = Option<FieldType>;

    fn selected(&self) -> Option<FieldType> {
        self.selected_field.to_owned()
    }

    fn select(&mut self, field_type: Option<FieldType>) {
        self.selected_field = field_type;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_move_to_next_field() {
        let mut field_context = Fields::new(&TerminalSize::Medium);
        field_context.select(Some(FieldType::Alias));

        field_context.next();
        assert_eq!(field_context.selected(), Some(FieldType::Namespace));
        assert_eq!(field_context.state.items[&FieldType::Alias].in_focus, false);
        assert_eq!(
            field_context.state.items[&FieldType::Namespace].in_focus,
            true
        );

        field_context.next();
        assert_eq!(field_context.selected(), Some(FieldType::Command));
        assert_eq!(
            field_context.state.items[&FieldType::Namespace].in_focus,
            false
        );
        assert_eq!(
            field_context.state.items[&FieldType::Command].in_focus,
            true
        );
    }

    #[test]
    fn should_move_to_previous_field() {
        let mut field_context = Fields::new(&TerminalSize::Medium);
        field_context.select(Some(FieldType::Alias));

        field_context.previous();
        assert_eq!(field_context.selected(), Some(FieldType::Tags));
        assert_eq!(field_context.state.items[&FieldType::Alias].in_focus, false);
        assert_eq!(field_context.state.items[&FieldType::Tags].in_focus, true);

        field_context.previous();
        assert_eq!(field_context.selected(), Some(FieldType::Description));
        assert_eq!(field_context.state.items[&FieldType::Tags].in_focus, false);
        assert_eq!(
            field_context.state.items[&FieldType::Description].in_focus,
            true
        );
    }

    #[test]
    fn should_return_the_selected_field() {
        let mut field_context = Fields::new(&TerminalSize::Medium);

        field_context.select(Some(FieldType::Namespace));
        let selected_field = field_context.selected_field_mut();
        assert_eq!(selected_field.unwrap().field_type(), FieldType::Namespace);
    }
}
