use super::{context::Context, popup::PopUp};
use crate::{command_item::CommandItem, commands::Commands, gui::layouts::view_mode::ViewMode};
use anyhow::Result;
use tui::widgets::ListState;

#[derive(Clone)]
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
    pub to_be_executed: Option<CommandItem>,
}

impl State {
    pub fn init(commands: Commands) -> State {
        //TODO colocar esses itens como static????
        let insert_menu_items = vec![
            (String::from("alias"), true),
            (String::from("namespace"), false),
            (String::from("command"), false),
            (String::from("description"), false),
            (String::from("tags"), false),
        ];
        let mut state = State {
            should_quit: false,
            commands_state: ListState::default(),
            namespace_state: ListState::default(),
            commands: commands.clone(),
            namespaces: Default::default(),
            current_namespace: String::from("All"),
            view_mode: ViewMode::List,
            context: Context::new(insert_menu_items),
            popup: PopUp::init(),
            show_help: false,
            to_be_executed: None,
        };
        state.load_namespaces();
        state.commands_state.select(Some(0));
        state.namespace_state.select(Some(0));
        state.context.focus_state.select(Some(0));
        state.context.current_command = match commands.clone().get_ref().get_command_item_ref(0) {
            Some(value) => Some(value.to_owned()),
            None => None,
        };
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

    pub fn reload_namespaces(&mut self) {
        self.load_namespaces();
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

    pub fn filtered_commands(&mut self) -> Vec<CommandItem> {
        if self.current_namespace == "All" {
            self.commands.all_commands()
        } else {
            self.commands
                .commands_from_namespace(self.current_namespace.clone())
                .expect("cannot save a command without an namespace")
        }
    }

    pub fn execute_callback_command(&self) -> Result<()> {
        match &self.to_be_executed {
            Some(command) => self.commands.exec_command(command)?,
            None => return Ok(()), //there is nothing to handle
        }

        Ok(())
    }
}
