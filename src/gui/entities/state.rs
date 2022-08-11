use super::{context::Context, field::Field, popup::PopUp};
use crate::{command::Command, commands::Commands, gui::layouts::view_mode::ViewMode};
use anyhow::Result;
use itertools::Itertools;
use tui::widgets::ListState;

pub struct State {
    pub should_quit: bool,
    pub commands_state: ListState,
    pub namespace_state: ListState,
    pub commands: Commands,
    pub namespaces: Vec<String>,
    pub current_namespace: String,
    pub view_mode: ViewMode,
    pub context: Context,
    pub popup: PopUp,
    pub show_help: bool,
    pub to_be_executed: Option<Command>,
    pub query_box: Field,
}

impl State {
    pub fn init(commands: Commands) -> State {
        let mut state = State {
            should_quit: false,
            commands_state: ListState::default(),
            namespace_state: ListState::default(),
            commands: commands.clone(),
            namespaces: Default::default(),
            current_namespace: String::from("All"),
            view_mode: ViewMode::Main,
            context: Context::new(),
            popup: PopUp::init(),
            show_help: false,
            to_be_executed: None,
            query_box: Field::new(
                String::from("query_box"),
                String::from("Find"),
                super::field::FieldType::QueryBox,
                false,
            ),
        };

        state.load_namespaces();
        state.commands_state.select(Some(0));
        state.namespace_state.select(Some(0));
        state.context.focus_state.select(Some(0));
        state.context.current_command = commands
            .get_command_item_ref(0)
            .map(|value| value.to_owned());
        state.popup.options_state.select(Some(0));

        state
    }

    pub fn load_namespaces(&mut self) {
        self.namespace_state.select(Some(0));
        self.namespaces = self.commands.namespaces();
        self.current_namespace = self.namespaces[0].clone();
        self.filtered_namespaces()
    }

    pub fn reload_state(&mut self) {
        self.load_namespaces();
        self.commands_state.select(Some(0));
    }

    pub fn next_command_item(&mut self) {
        let i = match self.commands_state.selected() {
            Some(i) => {
                if self.filtered_commands().is_empty() || i >= self.filtered_commands().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.commands_state.select(Some(i));
    }

    pub fn previous_command_item(&mut self) {
        let i = match self.commands_state.selected() {
            Some(i) => {
                if i == 0 && !self.filtered_commands().is_empty() {
                    self.filtered_commands().len() - 1
                } else if i == 0 && self.filtered_commands().is_empty() {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.commands_state.select(Some(i));
    }

    pub fn next_namespace(&mut self) {
        let i = match self.namespace_state.selected() {
            Some(i) => {
                if i >= self.namespaces.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.namespace_state.select(Some(i));
        self.current_namespace = String::from(self.namespaces.get(i).unwrap());
        self.commands_state.select(Some(0));
    }

    pub fn previous_namespace(&mut self) {
        let i = match self.namespace_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.namespaces.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.namespace_state.select(Some(i));
        self.current_namespace = String::from(self.namespaces.get(i).unwrap());
        self.commands_state.select(Some(0));
    }

    pub fn filtered_commands(&mut self) -> Vec<Command> {
        if let Ok(commands) = self
            .commands
            .commands(self.current_namespace.clone(), self.query_box.input.clone())
        {
            commands
        } else {
            vec![]
        }
    }

    pub fn filtered_namespaces(&mut self) {
        self.namespaces = self
            .filtered_commands()
            .iter()
            .map(|command| command.namespace.clone())
            .unique()
            .collect();
        self.namespaces.insert(0, String::from("All"));
        self.namespaces.sort();
    }

    pub fn execute_callback_command(&self) -> Result<()> {
        if let Some(command) = &self.to_be_executed {
            self.commands.exec_command(command)?;
        }

        Ok(())
    }
}
