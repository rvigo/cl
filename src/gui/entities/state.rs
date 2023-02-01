use crate::{
    command::Command,
    commands::Commands,
    gui::{
        layouts::ViewMode,
        widgets::{
            contexts::{field_context::FieldContext, popup_context::PopupContext},
            query_box::QueryBox,
        },
    },
};
use anyhow::Result;
use itertools::Itertools;
use std::{thread, time::Duration};
use tui::widgets::ListState;

pub struct State<'a> {
    pub should_quit: bool,
    pub commands_state: ListState,
    pub namespace_state: ListState,
    pub commands: Commands,
    pub namespaces: Vec<String>,
    pub current_namespace: String,
    pub view_mode: ViewMode,
    pub form_fields_context: FieldContext<'a>,
    pub popup_context: PopupContext<'a>,
    pub show_help: bool,
    pub to_be_executed: Option<Command>,
    pub query_box: QueryBox<'a>,
}

impl<'a> State<'a> {
    pub fn init(commands: Commands) -> State<'a> {
        let mut state = State {
            should_quit: false,
            commands_state: ListState::default(),
            namespace_state: ListState::default(),
            commands: commands.clone(),
            namespaces: Default::default(),
            current_namespace: String::from("All"),
            view_mode: ViewMode::default(),
            form_fields_context: FieldContext::default(),
            popup_context: PopupContext::default(),
            show_help: false,
            to_be_executed: None,
            query_box: QueryBox::default(),
        };

        state.load_namespaces();
        state.commands_state.select(Some(0));
        state.namespace_state.select(Some(0));
        state.form_fields_context.focus_state.select(Some(0));
        state.form_fields_context.select_command(
            commands
                .get_command_item_ref(0)
                .map(|value| value.to_owned()),
        );
        state.popup_context.choices_state.select(Some(0));

        state
    }

    pub fn load_namespaces(&mut self) {
        self.namespace_state.select(Some(0));
        self.namespaces = self.commands.namespaces();
        self.current_namespace = self.namespaces[0].clone();
        self.filter_namespaces()
    }

    pub fn reload_state(&mut self) {
        self.load_namespaces();
        self.commands_state.select(Some(0));
    }

    pub fn next_command(&mut self) {
        let mut i = self.commands_state.selected().unwrap_or(0);
        if !self.filter_commands().is_empty() {
            i = if i >= self.filter_commands().len() - 1 {
                0
            } else {
                i + 1
            };
        }
        self.commands_state.select(Some(i));
    }

    pub fn previous_command(&mut self) {
        let mut i = self.commands_state.selected().unwrap_or(0);
        if !self.filter_commands().is_empty() {
            i = if i == 0 {
                self.filter_commands().len() - 1
            } else {
                i - 1
            };
        };

        self.commands_state.select(Some(i));
    }

    pub fn next_namespace(&mut self) {
        let i = self.namespace_state.selected().unwrap_or(0);
        let i = if i >= self.namespaces.len() - 1 {
            0
        } else {
            i + 1
        };
        self.namespace_state.select(Some(i));
        self.current_namespace = self
            .namespaces
            .get(i)
            .unwrap_or(&String::from("All"))
            .to_owned();
        self.commands_state.select(Some(0));
    }

    pub fn previous_namespace(&mut self) {
        let i = self.namespace_state.selected().unwrap_or(0);
        let i = if i == 0 {
            self.namespaces.len() - 1
        } else {
            i - 1
        };

        self.namespace_state.select(Some(i));
        self.current_namespace = self
            .namespaces
            .get(i)
            .unwrap_or(&String::from("All"))
            .to_owned();
        self.commands_state.select(Some(0));
    }

    pub fn filter_commands(&mut self) -> Commands {
        let query_string = self.query_box.get_input();

        if let Ok(commands) = self
            .commands
            .commands(self.current_namespace.clone(), query_string)
        {
            commands
        } else {
            Commands::default()
        }
    }

    pub fn filter_namespaces(&mut self) {
        let filtered_commands = self.filter_commands();
        let unique_namespaces = filtered_commands
            .iter()
            .map(|command: &Command| &command.namespace)
            .unique()
            .cloned()
            .collect();
        self.namespaces = unique_namespaces;
        self.namespaces.sort();
        self.namespaces.insert(0, String::from("All"));
    }

    pub fn execute_callback_command(&self) -> Result<()> {
        if let Some(command) = &self.to_be_executed {
            if command.command.contains("#{") {
                eprintln!(
                    "Warning: This command appears to contains one or more named parameters placeholders. \
                    It may not run correctly using the interface.\n\
                If you want to use these parameters, please use the CLI option (cl exec --help)"
                );

                let seconds_to_sleep = Duration::from_secs(3);
                thread::sleep(seconds_to_sleep);

                eprintln!();
            }

            self.commands.exec_command(command, false, false)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn commands_builder(n_of_commands: usize) -> Commands {
        let mut commands = vec![];
        for i in 0..n_of_commands {
            commands.push(Command {
                namespace: format!("namespace{}", (i + 1)),
                command: "command".to_string(),
                description: None,
                alias: "alias".to_string(),
                tags: None,
            })
        }

        Commands::init(commands)
    }
    fn state_builder(n_of_commands: usize) -> State<'static> {
        let commands = commands_builder(n_of_commands);
        State::init(commands)
    }

    #[test]
    fn should_filter_namespaces() {
        let mut state = state_builder(1);

        state.filter_namespaces();

        let expected = vec!["All".to_string(), "namespace1".to_string()];
        assert_eq!(state.namespaces, expected);
    }

    #[test]
    fn should_load_namespaces() {
        let mut state = state_builder(1);

        state.load_namespaces();

        assert_eq!(state.namespace_state.selected().unwrap(), 0);
        assert_eq!(
            state.current_namespace,
            state.commands.namespaces()[0].clone()
        );
        assert_eq!(state.namespaces, state.commands.namespaces());
    }

    #[test]
    fn should_go_to_previous_namespace() {
        let mut state = state_builder(2);

        assert_eq!(state.current_namespace, "All");

        state.previous_namespace();
        assert_eq!(state.namespace_state.selected().unwrap(), 2);
        assert_eq!(state.current_namespace, "namespace2");
        assert_eq!(state.commands_state.selected().unwrap(), 0);

        state.previous_namespace();
        assert_eq!(state.namespace_state.selected().unwrap(), 1);
        assert_eq!(state.current_namespace, "namespace1");
        assert_eq!(state.commands_state.selected().unwrap(), 0);

        state.previous_namespace();
        assert_eq!(state.namespace_state.selected().unwrap(), 0);
        assert_eq!(state.current_namespace, "All");
        assert_eq!(state.commands_state.selected().unwrap(), 0);
    }

    #[test]
    fn should_go_to_next_namespace() {
        let mut state = state_builder(2);

        assert_eq!(state.current_namespace, "All");

        state.next_namespace();
        assert_eq!(state.namespace_state.selected().unwrap(), 1);
        assert_eq!(state.current_namespace, "namespace1");
        assert_eq!(state.commands_state.selected().unwrap(), 0);

        state.next_namespace();
        assert_eq!(state.namespace_state.selected().unwrap(), 2);
        assert_eq!(state.current_namespace, "namespace2");
        assert_eq!(state.commands_state.selected().unwrap(), 0);

        state.next_namespace();
        assert_eq!(state.namespace_state.selected().unwrap(), 0);
        assert_eq!(state.current_namespace, "All");
        assert_eq!(state.commands_state.selected().unwrap(), 0);
    }

    #[test]
    fn should_go_to_next_command() {
        let mut state = state_builder(3);

        assert_eq!(state.commands_state.selected(), Some(0));

        state.next_command();
        assert_eq!(state.commands_state.selected(), Some(1));

        state.next_command();
        assert_eq!(state.commands_state.selected(), Some(2));

        state.next_command();
        assert_eq!(state.commands_state.selected(), Some(0));
    }
    #[test]
    fn should_go_to_previous_command() {
        let mut state = state_builder(3);

        assert_eq!(state.commands_state.selected(), Some(0));

        state.previous_command();
        assert_eq!(state.commands_state.selected(), Some(2));

        state.previous_command();
        assert_eq!(state.commands_state.selected(), Some(1));
    }
}
