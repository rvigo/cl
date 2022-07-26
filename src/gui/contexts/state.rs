use super::{context::Context, popup::PopUp};
use crate::{command_item::CommandItem, commands::Commands, gui::layouts::view_mode::ViewMode};
use anyhow::Result;
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
    pub to_be_executed: Option<CommandItem>,
}

impl State {
    pub fn init(commands: Commands) -> State {
        //TODO colocar esses itens como static????
        let insert_menu_items = vec![
            (String::from("alias"), String::from(" Alias "), true),
            (
                String::from("namespace"),
                String::from(" Namespace "),
                false,
            ),
            (String::from("command"), String::from(" Command "), false),
            (
                String::from("description"),
                String::from(" Description "),
                false,
            ),
            (String::from("tags"), String::from(" Tags "), false),
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

    pub fn filtered_commands(&mut self) -> Vec<CommandItem> {
        if self.current_namespace.eq("All") {
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
        }
    }

    pub fn execute_callback_command(&self) -> Result<()> {
        if let Some(command) = &self.to_be_executed {
            self.commands.exec_command(command)?;
        }

        Ok(())
    }
}
