use super::Selectable;
use crate::{
    command::{Command, CommandBuilder},
    gui::{
        entities::states::{field_state::FieldState, State},
        screens::{
            widgets::{
                fields::Fields,
                text_field::{FieldType, TextField},
                WidgetKeyHandler,
            },
            ScreenSize,
        },
    },
};
use crossterm::event::KeyEvent;
use itertools::Itertools;

#[derive(Default)]
pub struct FieldContext<'a> {
    fields: Fields<'a>,
    state: FieldState,
    selected_command: Option<Command>,
}

impl<'a> FieldContext<'a> {
    pub fn order_field_by_size(&mut self, size: &ScreenSize) {
        self.fields.reorder_by_screen_size(size);
    }

    pub fn get_fields_iter(&self) -> impl Iterator<Item = TextField<'_>> {
        self.fields.get_fields_iter()
    }

    pub fn get_field_state_mut(&mut self) -> &mut FieldState {
        &mut self.state
    }

    pub fn selected_field_mut(&mut self) -> Option<&mut TextField<'a>> {
        self.state
            .selected()
            .and_then(|selected| self.fields.get_mut(&selected))
    }

    pub fn build_new_command(&mut self) -> Command {
        let mut command_builder = CommandBuilder::default();
        self.fields
            .iter_mut()
            .for_each(|(field_type, field)| match field_type {
                FieldType::Alias => {
                    command_builder.alias(field.text());
                }
                FieldType::Command => {
                    command_builder.command(field.text());
                }
                FieldType::Description => {
                    command_builder.description(field.text().to_option());
                }
                FieldType::Namespace => {
                    command_builder.namespace(field.text());
                }
                FieldType::Tags => {
                    command_builder.tags(
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

        command_builder.build()
    }

    pub fn build_edited_command(&mut self) -> Command {
        if let Some(command) = self.selected_command.as_mut() {
            self.fields
                .iter_mut()
                .for_each(|(field_type, field)| match field_type {
                    FieldType::Alias => command.alias = field.text(),
                    FieldType::Command => command.command = field.text(),
                    FieldType::Namespace => command.namespace = field.text(),
                    FieldType::Description => command.description = field.text().to_option(),
                    FieldType::Tags => {
                        command.tags = field
                            .text()
                            .split(',')
                            .map(|tag| String::from(tag.trim()))
                            .filter(|tag| !tag.is_empty())
                            .collect_vec()
                            .to_option();
                    }
                });

            return command.to_owned();
        }
        Command::default()
    }

    pub fn selected_command(&self) -> Option<&Command> {
        self.selected_command.as_ref()
    }

    pub fn select_command(&mut self, selected_command: Option<Command>) {
        self.selected_command = selected_command
    }

    pub fn popuplate_form(&mut self) {
        if let Some(current_command) = self.selected_command.as_ref() {
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
                self.state.update_field(field);
            });
        }
    }

    pub fn handle_input(&mut self, input: KeyEvent) {
        if let Some(selected) = self.state.selected() {
            if let Some(field) = self.fields.get_mut(&selected) {
                field.handle_input(input);
                self.state.updated_edited_field(&field.to_owned())
            }
        }
    }

    pub fn clear_inputs(&mut self) {
        self.reset_edition_state();
        self.fields.clear_inputs()
    }

    pub fn clear_selection(&mut self) {
        if let Some(selected) = self.selected_field_mut() {
            selected.deactivate_focus()
        }
    }

    pub fn is_modified(&self) -> bool {
        self.state.is_modified()
    }

    fn reset_edition_state(&mut self) {
        self.state.reset_fields_edition_state()
    }
}

impl Selectable for FieldContext<'_> {
    fn next(&mut self) {
        if let Some(current_field_type) = self.state.selected() {
            if let Some(field) = self.fields.get_field_mut(&current_field_type) {
                field.deactivate_focus()
            }

            let order = self.fields.get_order();

            if let Some(pos) = order.iter().position(|x| current_field_type.eq(x)) {
                let new_field_idx = (pos + 1) % order.len();
                let new_field_type = self.fields.get_order()[new_field_idx].to_owned();

                // selects the new field type
                self.state.select(Some(new_field_type));
                if let Some(new_field_type) = self.state.selected() {
                    if let Some(field) = self.fields.get_field_mut(&new_field_type) {
                        field.activate_focus()
                    }
                }
            };
        }
    }

    fn previous(&mut self) {
        if let Some(current_field_type) = self.state.selected() {
            if let Some(field) = self.fields.get_field_mut(&current_field_type) {
                field.deactivate_focus()
            }

            let order = self.fields.get_order();
            if let Some(pos) = order.iter().position(|x| current_field_type.eq(x)) {
                let new_field_idx = (pos + order.len() - 1) % order.len();
                let new_field_type = self.fields.get_order()[new_field_idx].to_owned();

                // selects the new field type
                self.state.select(Some(new_field_type));
                if let Some(new_field_type) = self.state.selected() {
                    if let Some(field) = self.fields.get_field_mut(&new_field_type) {
                        field.activate_focus()
                    }
                }
            }
        }
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
    use crate::gui::screens::widgets::WidgetKeyHandler;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

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

        Fields::from((map, order))
    }

    #[test]
    fn should_move_to_next_field() {
        let mut field_context = FieldContext::default();
        field_context.state.select(Some(FieldType::Alias));

        field_context.next();
        assert_eq!(field_context.state.selected(), Some(FieldType::Namespace));
        assert_eq!(field_context.fields[&FieldType::Alias].in_focus(), false);
        assert_eq!(field_context.fields[&FieldType::Namespace].in_focus(), true);

        field_context.next();
        assert_eq!(field_context.state.selected(), Some(FieldType::Command));
        assert_eq!(
            field_context.fields[&FieldType::Namespace].in_focus(),
            false
        );
        assert_eq!(field_context.fields[&FieldType::Command].in_focus(), true);
    }

    #[test]
    fn should_move_to_previous_field() {
        let mut field_context = FieldContext::default();
        field_context.state.select(Some(FieldType::Alias));

        field_context.previous();
        assert_eq!(field_context.state.selected(), Some(FieldType::Tags));
        assert_eq!(field_context.fields[&FieldType::Alias].in_focus(), false);
        assert_eq!(field_context.fields[&FieldType::Tags].in_focus(), true);

        field_context.previous();
        assert_eq!(field_context.state.selected(), Some(FieldType::Description));
        assert_eq!(field_context.fields[&FieldType::Tags].in_focus(), false);
        assert_eq!(
            field_context.fields[&FieldType::Description].in_focus(),
            true
        );
    }

    #[test]
    fn should_return_the_selected_field() {
        let mut field_context = FieldContext::default();

        field_context.state.select(Some(FieldType::Namespace));
        let selected_field = field_context.selected_field_mut();
        assert_eq!(selected_field.unwrap().field_type(), FieldType::Namespace);
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
        field_context.popuplate_form();

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
