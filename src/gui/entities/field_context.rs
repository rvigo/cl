use super::{field_state::FieldState, state::State};
use crate::{
    command::{Command, CommandBuilder},
    gui::{
        layouts::TerminalSize,
        widgets::{
            fields::Fields,
            text_field::{FieldType, TextField},
        },
    },
};
use itertools::Itertools;
use tui_textarea::{
    CursorMove::{Bottom, End},
    TextArea,
};

#[derive(Default, Clone)]
pub struct FieldContext<'a> {
    fields: Fields<'a>,
    focus_state: FieldState,
    selected_command: Option<Command>,
}

impl<'a> FieldContext<'a> {
    pub fn order_field_by_size(&mut self, size: &TerminalSize) {
        self.fields.reorder(size);
    }

    pub fn get_fields(&self) -> Vec<TextField<'_>> {
        self.fields.get_fields()
    }

    pub fn get_focus_state_mut(&mut self) -> &mut FieldState {
        &mut self.focus_state
    }

    pub fn next_field(&mut self) {
        if let Some(current_field_type) = self.focus_state.selected() {
            if let Some(field) = self.fields.get_field_mut(&current_field_type) {
                field.deactivate_focus()
            }

            let order = self.fields.get_order();

            if let Some(pos) = order.iter().position(|x| current_field_type.eq(x)) {
                let new_field_idx = if pos >= order.len() - 1 { 0 } else { pos + 1 };
                let new_field_type = self.fields.get_order()[new_field_idx].to_owned();

                // selects the new field type
                self.focus_state.select(Some(new_field_type));
                if let Some(new_field_type) = self.focus_state.selected() {
                    if let Some(field) = self.fields.get_field_mut(&new_field_type) {
                        field.activate_focus()
                    }
                }
            };
        }
    }

    pub fn previous_field(&mut self) {
        if let Some(current_field_type) = self.focus_state.selected() {
            if let Some(field) = self.fields.get_field_mut(&current_field_type) {
                field.deactivate_focus()
            }

            let order = self.fields.get_order();
            if let Some(pos) = order.iter().position(|x| current_field_type.eq(x)) {
                let new_field_idx = if pos == 0 {
                    self.fields.len() - 1
                } else {
                    pos - 1
                };
                let new_field_type = self.fields.get_order()[new_field_idx].to_owned();

                // selects the new field type
                self.focus_state.select(Some(new_field_type));
                if let Some(new_field_type) = self.focus_state.selected() {
                    if let Some(field) = self.fields.get_field_mut(&new_field_type) {
                        field.activate_focus()
                    }
                }
            }
        }
    }

    pub fn selected_field(&mut self) -> Option<&mut TextField<'a>> {
        if let Some(selected) = self.focus_state.selected() {
            self.fields.get_mut(&selected)
        } else {
            None
        }
    }

    pub fn build_new_command(&mut self) -> Command {
        let mut command_builder = CommandBuilder::default();
        self.fields
            .iter_mut()
            .for_each(|(field_type, field)| match field_type {
                FieldType::Alias => {
                    command_builder.alias(field.input_as_string());
                }
                FieldType::Command => {
                    command_builder.command(field.input_as_string());
                }
                FieldType::Tags => {
                    if field.text_area.is_empty() {
                        command_builder.tags(None::<Vec<&str>>);
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
                        command_builder.description(None::<&str>);
                    } else {
                        command_builder.description(Some(field.input_as_string()));
                    }
                }
                FieldType::Namespace => {
                    command_builder.namespace(field.input_as_string());
                }
            });

        command_builder.build()
    }

    pub fn edit_command(&mut self) -> Command {
        let mut command = self
            .selected_command()
            .map(|command| command.to_owned())
            .unwrap();
        self.fields
            .iter_mut()
            .for_each(|(field_type, field)| match field_type {
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

        command
    }

    pub fn selected_command(&self) -> Option<&Command> {
        self.selected_command.as_ref()
    }

    pub fn select_command(&mut self, selected_command: Option<Command>) {
        self.selected_command = selected_command
    }

    pub fn set_selected_command_input(&mut self) {
        let selected_command = self.selected_command.as_mut();
        if let Some(current_command) = selected_command {
            self.fields.iter_mut().for_each(|(field_type, field)| {
                match field_type {
                    FieldType::Alias => {
                        field.text_area = TextArea::from(vec![current_command.alias.clone()]);
                        field.text_area.move_cursor(Bottom);
                        field.text_area.move_cursor(End);
                    }
                    FieldType::Command => {
                        field.text_area = TextArea::from(
                            current_command
                                .command
                                .clone()
                                .lines()
                                .map(String::from)
                                .collect::<Vec<String>>(),
                        );
                        field.text_area.move_cursor(Bottom);
                        field.text_area.move_cursor(End);
                    }
                    FieldType::Namespace => {
                        field.text_area = TextArea::from(vec![current_command.namespace.clone()]);
                        field.text_area.move_cursor(Bottom);
                        field.text_area.move_cursor(End);
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
                        field.text_area.move_cursor(Bottom);
                        field.text_area.move_cursor(End);
                    }
                    FieldType::Tags => {
                        field.text_area = TextArea::from(vec![current_command
                            .tags
                            .as_ref()
                            .unwrap_or(&vec![String::from("")])
                            .join(", ")]);
                        field.text_area.move_cursor(Bottom);
                        field.text_area.move_cursor(End);
                    }
                };
            });
        }
    }

    pub fn clear_inputs(&mut self) {
        self.fields.clear_inputs()
    }

    pub fn clear_selection(&mut self) {
        if let Some(selected) = self.selected_field() {
            selected.deactivate_focus()
        }
    }
}

#[cfg(test)]
mod test {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::*;
    fn create_fields() -> Fields<'static> {
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

        alias.on_input(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        alias.on_input(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
        alias.on_input(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
        alias.on_input(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        alias.on_input(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
        namespace.on_input(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
        command.on_input(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
        // multifield description field
        description.on_input(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        description.on_input(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        description.on_input(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        tags.on_input(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE));

        let map = vec![alias, namespace, command, description, tags]
            .into_iter()
            .map(|f| (f.field_type.to_owned(), f))
            .collect();
        let order = [
            FieldType::Alias,
            FieldType::Namespace,
            FieldType::Command,
            FieldType::Description,
            FieldType::Tags,
        ]
        .to_vec();

        Fields::from((map, order))
    }

    #[test]
    fn should_move_to_next_field() {
        let mut field_context = FieldContext::default();
        field_context.focus_state.select(Some(FieldType::Alias));

        field_context.next_field();
        assert_eq!(
            field_context.focus_state.selected(),
            Some(FieldType::Namespace)
        );
        assert_eq!(field_context.fields[&FieldType::Alias].in_focus(), false);
        assert_eq!(field_context.fields[&FieldType::Namespace].in_focus(), true);

        field_context.next_field();
        assert_eq!(
            field_context.focus_state.selected(),
            Some(FieldType::Command)
        );
        assert_eq!(
            field_context.fields[&FieldType::Namespace].in_focus(),
            false
        );
        assert_eq!(field_context.fields[&FieldType::Command].in_focus(), true);
    }

    #[test]
    fn should_move_to_previous_field() {
        let mut field_context = FieldContext::default();
        field_context.focus_state.select(Some(FieldType::Alias));

        field_context.previous_field();
        assert_eq!(field_context.focus_state.selected(), Some(FieldType::Tags));
        assert_eq!(field_context.fields[&FieldType::Alias].in_focus(), false);
        assert_eq!(field_context.fields[&FieldType::Tags].in_focus(), true);

        field_context.previous_field();
        assert_eq!(
            field_context.focus_state.selected(),
            Some(FieldType::Description)
        );
        assert_eq!(field_context.fields[&FieldType::Tags].in_focus(), false);
        assert_eq!(
            field_context.fields[&FieldType::Description].in_focus(),
            true
        );
    }

    #[test]
    fn should_return_the_selected_field() {
        let mut field_context = FieldContext::default();

        field_context.focus_state.select(Some(FieldType::Namespace));
        let selected_field = field_context.selected_field();
        assert_eq!(selected_field.unwrap().field_type, FieldType::Namespace);
    }

    #[test]
    fn should_build_a_new_command() {
        let mut field_context = FieldContext::default();
        field_context.fields = create_fields();
        let command = field_context.build_new_command();

        assert!(command.validate().is_ok());
        assert_eq!(command.alias, "alias");
        assert_eq!(command.command, "c");
        assert_eq!(command.namespace, "n");
        assert_eq!(command.description, Some("d\nd".to_string()));
        assert_eq!(command.tags, Some(vec!["t".to_string(),]));
    }

    #[test]
    fn should_set_input_based_at_selected_command() {
        let mut field_context = FieldContext::default();
        let selected_command = Command {
            alias: String::from("alias"),
            command: String::from("command"),
            namespace: String::from("namespace"),
            description: None,
            tags: Some(vec![String::from("tag1"), String::from("tag2")]),
        };
        field_context.select_command(Some(selected_command));
        field_context.set_selected_command_input();

        let command = field_context.selected_command();

        assert!(command.is_some());
        let command = command.unwrap();

        assert_eq!(command.alias, "alias");
        assert_eq!(command.command, "command");
        assert_eq!(command.namespace, "namespace");
        assert_eq!(command.description, None);
        assert_eq!(
            command.tags,
            Some(vec![String::from("tag1"), String::from("tag2")])
        );
    }
}
