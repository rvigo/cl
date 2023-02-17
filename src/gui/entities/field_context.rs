use crate::{
    command::{Command, CommandBuilder},
    gui::widgets::{
        field::{Field, FieldType},
        fields::Fields,
    },
};
use itertools::Itertools;
use tui::widgets::ListState;
use tui_textarea::{
    CursorMove::{Bottom, End},
    TextArea,
};

#[derive(Default)]
pub struct FieldContext<'a> {
    fields: Fields<'a>,
    focus_state: ListState,
    selected_command: Option<Command>,
}

impl<'a> FieldContext<'a> {
    pub fn get_fields(&self) -> &Fields {
        &self.fields
    }

    pub fn get_fields_mut(&mut self) -> &mut Fields<'a> {
        &mut self.fields
    }

    pub fn get_focus_state(&self) -> ListState {
        self.focus_state.to_owned()
    }

    pub fn next_field(&mut self) {
        let old_idx = self.focus_state.selected().unwrap_or(0);
        if let Some(old_field) = self.fields.get_mut(old_idx) {
            old_field.toggle_focus()
        };

        let mut idx = self.focus_state.selected().unwrap_or(0);
        idx = if idx >= self.fields.len() - 1 {
            0
        } else {
            idx + 1
        };

        self.focus_state.select(Some(idx));
        if let Some(new_field) = self.fields.get_mut(idx) {
            new_field.toggle_focus()
        };
    }

    pub fn previous_field(&mut self) {
        let old_idx = self.focus_state.selected().unwrap_or(0);
        if let Some(old_field) = self.fields.get_mut(old_idx) {
            old_field.toggle_focus()
        };

        let mut idx = self.focus_state.selected().unwrap_or(0);
        idx = if idx == 0 {
            self.fields.len() - 1
        } else {
            idx - 1
        };

        self.focus_state.select(Some(idx));
        if let Some(new_field) = self.fields.get_mut(idx) {
            new_field.toggle_focus()
        };
    }

    pub fn selected_field_mut(&mut self) -> Option<&mut Field<'a>> {
        let idx = self.focus_state.selected().unwrap_or(0);
        self.fields.get_mut(idx)
    }

    pub fn build_new_command(&mut self) -> Command {
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

        command_builder.build()
    }

    pub fn edit_command(&mut self) -> Command {
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

        command
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
        if let Some(current_command) = self.selected_command.as_mut() {
            self.fields.iter_mut().for_each(|field| {
                match field.field_type {
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
}

#[cfg(test)]
mod test {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use super::*;
    fn create_fields() -> Fields<'static> {
        let mut alias = Field::new(String::from("alias"), FieldType::Alias, true, false);
        let mut command = Field::new(String::from("command"), FieldType::Command, false, false);
        let mut namespace = Field::new(
            String::from("namespace"),
            FieldType::Namespace,
            false,
            false,
        );
        let mut description = Field::new(
            String::from("description"),
            FieldType::Description,
            false,
            false,
        );
        let mut tags = Field::new(String::from("tags"), FieldType::Tags, false, false);

        alias.on_input(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        alias.on_input(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
        alias.on_input(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
        alias.on_input(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        alias.on_input(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
        namespace.on_input(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
        command.on_input(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
        description.on_input(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        tags.on_input(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE));

        return Fields(vec![alias, namespace, command, description, tags]);
    }
    #[test]
    fn should_move_to_next_field() {
        let mut field_context = FieldContext::default();
        let field1 = Field::new(String::from("alias"), FieldType::Alias, true, false);
        let field2 = Field::new(String::from("command"), FieldType::Command, false, false);
        field_context.fields.push(field1);
        field_context.fields.push(field2);
        field_context.focus_state.select(Some(0));

        field_context.next_field();
        assert_eq!(field_context.focus_state.selected(), Some(1));
        assert_eq!(field_context.fields[0].in_focus(), false);
        assert_eq!(field_context.fields[1].in_focus(), true);

        field_context.next_field();
        assert_eq!(field_context.focus_state.selected(), Some(0));
        assert_eq!(field_context.fields[0].in_focus(), true);
        assert_eq!(field_context.fields[1].in_focus(), false);
    }

    #[test]
    fn should_move_to_previous_field() {
        let mut field_context = FieldContext::default();
        let field1 = Field::new(String::from("alias"), FieldType::Alias, true, false);
        let field2 = Field::new(String::from("command"), FieldType::Command, false, false);
        field_context.fields.push(field1);
        field_context.fields.push(field2);
        field_context.focus_state.select(Some(0));

        field_context.previous_field();
        assert_eq!(field_context.focus_state.selected(), Some(1));
        assert_eq!(field_context.fields[0].in_focus(), false);
        assert_eq!(field_context.fields[1].in_focus(), true);

        field_context.previous_field();
        assert_eq!(field_context.focus_state.selected(), Some(0));
        assert_eq!(field_context.fields[0].in_focus(), true);
        assert_eq!(field_context.fields[1].in_focus(), false);
    }

    #[test]
    fn should_return_the_selected_field() {
        let mut field_context = FieldContext::default();
        let field1 = Field::new(String::from("alias"), FieldType::Alias, true, false);
        let field2 = Field::new(String::from("command"), FieldType::Command, false, false);
        field_context.fields.push(field1);
        field_context.fields.push(field2);
        field_context.focus_state.select(Some(0));

        field_context.focus_state.select(Some(1));
        let selected_field = field_context.selected_field_mut();
        assert_eq!(selected_field.unwrap().field_type, FieldType::Command);
    }

    #[test]
    fn shoud_clear_fields_input() {
        let mut field_context = FieldContext::default();
        let field1 = Field::new(String::from("alias"), FieldType::Alias, true, false);
        let field2 = Field::new(String::from("command"), FieldType::Command, false, false);
        field_context.fields.push(field1);
        field_context.fields.push(field2);

        field_context.fields.clear_fields_input();
        assert_eq!(field_context.fields[0].input_as_string(), "")
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
        assert_eq!(command.description, Some("d".to_string()));
        assert_eq!(command.tags, Some(vec!["t".to_string(),]));
    }

    #[test]
    fn should_set_input_based_at_selected_command() {
        let mut field_context = FieldContext::default();
        field_context.reset_fields();
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
