use super::Selectable;
use crate::{
    entities::{states::State, terminal::TerminalSize},
    widgets::{
        field_state::FieldState,
        text_field::{FieldType, TextField},
    },
};
use cl_core::{
    command::{Command, CommandBuilder},
    hashmap,
};
use crossterm::event::KeyEvent;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Fields<'fields> {
    fields: FieldState<'fields>,
    selected_field: Option<FieldType>,
    original_fields: HashMap<FieldType, String>,
    edited_fields: HashMap<FieldType, String>,
}

impl<'fields> Fields<'fields> {
    pub fn new(size: &TerminalSize) -> Self {
        Self {
            fields: FieldState::new(size),
            selected_field: None,
            original_fields: hashmap!(),
            edited_fields: hashmap!(),
        }
    }

    pub fn sort(&mut self, size: &TerminalSize) {
        self.fields.sort(size);
    }

    pub fn fields_iter(&self) -> impl Iterator<Item = TextField<'_>> {
        self.fields.fields_iter()
    }

    pub fn selected_field_mut(&mut self) -> Option<&mut TextField<'fields>> {
        self.selected()
            .and_then(|field| self.fields.get_mut(&field))
    }

    pub fn build_new_command(&mut self) -> Command {
        let mut new = CommandBuilder::default();

        self.fields
            .iter_mut()
            .for_each(|(field_type, field)| match field_type {
                FieldType::Alias => {
                    new.alias(field.text());
                }
                FieldType::Command => {
                    new.command(field.text());
                }
                FieldType::Description => {
                    new.description(field.text().to_option());
                }
                FieldType::Namespace => {
                    new.namespace(field.text());
                }
                FieldType::Tags => {
                    new.tags(
                        field
                            .text()
                            .split(',')
                            .map(|tag| String::from(tag.trim()))
                            .filter(|tag| !tag.is_empty())
                            .collect_vec()
                            .to_option(),
                    );
                }
            });

        new.build()
    }

    pub fn build_edited_command(&mut self) -> Command {
        let mut edited = CommandBuilder::default();

        self.fields
            .iter()
            .for_each(|(field_type, field)| match field_type {
                FieldType::Alias => {
                    edited.alias(field.text());
                }
                FieldType::Command => {
                    edited.command(field.text());
                }
                FieldType::Namespace => {
                    edited.namespace(field.text());
                }
                FieldType::Description => {
                    edited.description(field.text().to_option());
                }
                FieldType::Tags => {
                    edited.tags(
                        field
                            .text()
                            .split(',')
                            .map(|tag| String::from(tag.trim()))
                            .filter(|tag| !tag.is_empty())
                            .collect_vec()
                            .to_option(),
                    );
                }
            });

        edited.build()
    }

    pub fn popuplate_form(&mut self, current_command: &Command) {
        self.fields.iter_mut().for_each(|(field_type, field)| {
            match field_type {
                FieldType::Alias => {
                    field.set_text(current_command.alias.to_owned());
                }
                FieldType::Command => {
                    field.set_text(
                        current_command
                            .command
                            .lines()
                            .map(String::from)
                            .collect::<Vec<String>>(),
                    );
                }
                FieldType::Namespace => {
                    field.set_text(current_command.namespace.to_owned());
                }
                FieldType::Description => {
                    field.set_text(
                        current_command
                            .description
                            .as_ref()
                            .unwrap_or(&String::from(""))
                            .lines()
                            .map(String::from)
                            .collect::<Vec<String>>(),
                    );
                }
                FieldType::Tags => {
                    field.set_text(current_command.tags.as_ref().unwrap_or(&vec![]).join(", "));
                }
            };

            field.move_cursor_to_end_of_text();

            self.original_fields
                .insert(field.field_type(), field.text());

            self.edited_fields.insert(field.field_type(), field.text());
        });
    }

    pub fn handle_input(&mut self, input: KeyEvent) {
        if let Some(selected) = self.selected() {
            self.fields.handle_input(&selected, input);

            self.fields
                .get(&selected)
                .map(|field| self.edited_fields.insert(selected, field.text()));
        }
    }

    pub fn clear_inputs(&mut self) {
        let mut default_map = hashmap!();

        for field_type in FieldType::iter() {
            default_map.insert(field_type.to_owned(), String::default());
        }

        self.original_fields = default_map.to_owned();
        self.edited_fields = default_map;

        self.fields.clear_inputs()
    }

    pub fn reset(&mut self) {
        self.selected_field_mut().map(TextField::deactivate_focus);
        self.select(Some(FieldType::default()));
        self.selected_field_mut().map(TextField::activate_focus);
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
}

impl Selectable for Fields<'_> {
    fn next(&mut self) {
        if let Some(current_field_type) = self.selected() {
            self.fields
                .get_field_mut(&current_field_type)
                .map(TextField::deactivate_focus);

            let sequence = self.fields.get_sequence();

            if let Some(pos) = sequence.iter().position(|x| current_field_type.eq(x)) {
                let new_field_idx = (pos + 1) % sequence.len();
                let new_field_type = self.fields.get_sequence()[new_field_idx].to_owned();

                // selects the new field type
                self.select(Some(new_field_type));

                self.selected()
                    .and_then(|selected| self.fields.get_field_mut(&selected))
                    .map(TextField::activate_focus);
            };
        }
    }

    fn previous(&mut self) {
        if let Some(current_field_type) = self.selected() {
            if let Some(field) = self.fields.get_field_mut(&current_field_type) {
                field.deactivate_focus()
            }

            let order = self.fields.get_sequence();
            if let Some(pos) = order.iter().position(|x| current_field_type.eq(x)) {
                let new_field_idx = (pos + order.len() - 1) % order.len();
                let new_field_type = self.fields.get_sequence()[new_field_idx].to_owned();

                // selects the new field type
                self.select(Some(new_field_type));

                self.selected()
                    .and_then(|selected| self.fields.get_field_mut(&selected))
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

/// Convets a type into an Option
trait ToOption {
    fn to_option(&self) -> Option<Self>
    where
        Self: Sized;
}

impl ToOption for String {
    fn to_option(&self) -> Option<Self> {
        if self.is_empty() {
            None
        } else {
            Some(self.to_owned())
        }
    }
}

impl ToOption for Vec<String> {
    fn to_option(&self) -> Option<Self>
    where
        Self: Sized,
    {
        if self.is_empty() || self.iter().all(String::is_empty) {
            None
        } else {
            Some(self.to_owned())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{entities::terminal::TerminalSize, widgets::WidgetKeyHandler};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn create_fields() -> FieldState<'static> {
        let mut alias = TextField::new(String::from("alias"), FieldType::Alias, true, false);
        let mut command = TextField::new(String::from("command"), FieldType::Command, false, true);
        let mut namespace = TextField::new(
            String::from("namespace"),
            FieldType::Namespace,
            false,
            false,
        );
        let mut description = TextField::new(
            String::from("description"),
            FieldType::Description,
            false,
            true,
        );
        let mut tags = TextField::new(String::from("tags"), FieldType::Tags, false, false);

        alias.handle_input(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        alias.handle_input(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
        alias.handle_input(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
        alias.handle_input(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        alias.handle_input(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
        namespace.handle_input(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
        command.handle_input(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
        // multifield description field
        description.handle_input(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        description.handle_input(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        description.handle_input(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        tags.handle_input(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE));

        let map = vec![alias, namespace, command, description, tags]
            .into_iter()
            .map(|f| (f.field_type(), f))
            .collect();
        let order = [
            FieldType::Alias,
            FieldType::Namespace,
            FieldType::Command,
            FieldType::Description,
            FieldType::Tags,
        ]
        .to_vec();

        FieldState::from((map, order))
    }

    #[test]
    fn should_move_to_next_field() {
        let mut field_context = Fields::new(&TerminalSize::Medium);
        field_context.select(Some(FieldType::Alias));

        field_context.next();
        assert_eq!(field_context.selected(), Some(FieldType::Namespace));
        assert_eq!(field_context.fields[&FieldType::Alias].in_focus(), false);
        assert_eq!(field_context.fields[&FieldType::Namespace].in_focus(), true);

        field_context.next();
        assert_eq!(field_context.selected(), Some(FieldType::Command));
        assert_eq!(
            field_context.fields[&FieldType::Namespace].in_focus(),
            false
        );
        assert_eq!(field_context.fields[&FieldType::Command].in_focus(), true);
    }

    #[test]
    fn should_move_to_previous_field() {
        let mut field_context = Fields::new(&TerminalSize::Medium);
        field_context.select(Some(FieldType::Alias));

        field_context.previous();
        assert_eq!(field_context.selected(), Some(FieldType::Tags));
        assert_eq!(field_context.fields[&FieldType::Alias].in_focus(), false);
        assert_eq!(field_context.fields[&FieldType::Tags].in_focus(), true);

        field_context.previous();
        assert_eq!(field_context.selected(), Some(FieldType::Description));
        assert_eq!(field_context.fields[&FieldType::Tags].in_focus(), false);
        assert_eq!(
            field_context.fields[&FieldType::Description].in_focus(),
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

    #[test]
    fn should_build_a_new_command() {
        let mut field_context = Fields::new(&TerminalSize::Medium);

        field_context.fields = create_fields();
        let command = field_context.build_new_command();

        assert!(command.validate().is_ok());
        assert_eq!(command.alias, "alias");
        assert_eq!(command.command, "c");
        assert_eq!(command.namespace, "n");
        assert_eq!(command.description, Some("d\nd".to_string()));
        assert_eq!(command.tags, Some(vec!["t".to_string(),]));
    }

    // #[test]
    // fn should_set_input_based_at_selected_command() {
    //     let mut field_context = Fields::new(&TerminalSize::Medium);
    //     let selected_command = Command {
    //         alias: String::from("alias"),
    //         command: String::from("command"),
    //         namespace: String::from("namespace"),
    //         description: None,
    //         tags: Some(vec![String::from("tag1"), String::from("tag2")]),
    //     };
    //     field_context.select_command(Some(selected_command));
    //     field_context.popuplate_form();

    //     let command = field_context.selected_command();

    //     assert!(command.is_some());
    //     let command = command.unwrap();

    //     assert_eq!(command.alias, "alias");
    //     assert_eq!(command.command, "command");
    //     assert_eq!(command.namespace, "namespace");
    //     assert_eq!(command.description, None);
    //     assert_eq!(
    //         command.tags,
    //         Some(vec![String::from("tag1"), String::from("tag2")])
    //     );
    // }
}
