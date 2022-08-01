use super::{context::Context, popup::PopUp};
use crate::{
    command::{Command, CommandBuilder},
    commands::Commands,
    gui::layouts::view_mode::ViewMode,
};
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
    pub find_flag: bool,
    pub query_string: String,
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
            view_mode: ViewMode::List,
            context: Context::new(),
            popup: PopUp::init(),
            show_help: false,
            to_be_executed: None,
            find_flag: false,
            query_string: String::from(""),
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
        let mut ns = self.commands.namespaces();
        ns.insert(0, String::from("All"));
        self.current_namespace = String::from("All");
        self.namespaces = ns;
    }

    pub fn reload_state(&mut self) {
        self.load_namespaces();
        self.commands_state.select(Some(0));
    }

    pub fn next_command_item(&mut self) {
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

    pub fn previous_command_item(&mut self) {
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

    pub fn filtered_commands(&mut self) -> Vec<Command> {
        let filtered_commands = if self.current_namespace.eq("All") {
            self.commands.all_commands()
        } else {
            let namespaces = self
                .commands
                .commands_from_namespace(self.current_namespace.clone())
                .expect("cannot save a command without an namespace");

            if namespaces.is_empty() {
                self.commands.all_commands()
            } else {
                namespaces
            }
        };

        self.filter_commands_by_query_string(filtered_commands)
    }

    pub fn execute_callback_command(&self) -> Result<()> {
        if let Some(command) = &self.to_be_executed {
            self.commands.exec_command(command)?;
        }

        Ok(())
    }

    pub fn get_current_command(&mut self) -> Command {
        let idx = self
            .commands_state
            .selected()
            .expect("a command should always be selected");

        if self.filtered_commands().is_empty() {
            //creates an empty command
            return CommandBuilder::default().build();
        }

        self.filtered_commands().get(idx).unwrap().to_owned()
    }

    pub fn set_find_active(&mut self) {
        self.find_flag = true
    }

    pub fn set_find_deactive(&mut self) {
        self.find_flag = false
    }

    fn filter_commands_by_query_string(&mut self, commands: Vec<Command>) -> Vec<Command> {
        commands
            .iter()
            .filter(|command| {
                command.namespace.contains(&self.query_string)
                    || command.alias.contains(&self.query_string)
                    || command.tags_as_string().contains(&self.query_string)
                    || (command.description.is_some()
                        && command
                            .description
                            .as_ref()
                            .unwrap()
                            .contains(&self.query_string))
            })
            .map(|command| command.to_owned())
            .collect_vec()
    }
}
