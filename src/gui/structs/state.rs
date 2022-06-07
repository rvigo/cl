use tui::widgets::ListState;

use crate::{command_item::CommandItem, commands::Commands};

#[derive(Debug, Clone)]
pub struct State {
    pub commands_state: ListState,
    pub namespace_state: ListState,
    pub commands: Commands,
    pub namespaces: Vec<String>,
    pub current_namespace: String,
    pub current_command: Option<CommandItem>,
}

impl State {
    pub fn with_items(commands: Commands, namespaces: Vec<String>) -> State {
        let mut state = State {
            commands_state: ListState::default(),
            namespace_state: ListState::default(),
            commands: commands.clone(),
            namespaces,
            current_namespace: String::from("All"),
            current_command: match commands.clone().get_ref().get_command_item(0) {
                Some(value) => Some(value.to_owned()),
                None => None,
            },
        };
        state.commands_state.select(Some(0));
        state.namespace_state.select(Some(0));

        state
    }

    pub fn get_command_state_mut_ref(&mut self) -> &mut ListState {
        &mut self.commands_state
    }

    pub fn get_ref(&self) -> &State {
        self
    }

    pub fn get_mut_ref(&mut self) -> &mut State {
        self
    }

    pub fn next(&mut self) {
        let i = match self.commands_state.selected() {
            Some(i) => {
                if i >= self.filtered_commands().len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.commands_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.commands_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_commands().len() - 1
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
        self.current_namespace = String::from(self.namespaces.get(i).unwrap_or(&"All".to_string()));
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
        self.current_namespace = String::from(self.namespaces.get(i).unwrap_or(&"All".to_string()));
        self.commands_state.select(Some(0));
    }

    pub fn filtered_commands(&mut self) -> Vec<CommandItem> {
        self.commands
            .get_items_mut_ref()
            .clone()
            .into_iter()
            .filter(|c| {
                if self.current_namespace == "All" {
                    true
                } else {
                    c.namespace == self.current_namespace
                }
            })
            .collect::<Vec<CommandItem>>()
    }
}
