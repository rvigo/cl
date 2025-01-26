use cl_core::{
    fs, Command, CommandExec, CommandVec, CommandVecExt, Commands, Config, DefaultConfig,
};
use log::{debug, error};

pub struct State {
    pub commands: Commands<'static>,
    pub current_items: CommandVec<'static>,
    pub selected_command: SelectedCommand,
    pub selected_namespace: SelectedNamespace,
    pub namespaces: Vec<String>,
    config: DefaultConfig,
}

#[derive(Default, PartialEq, Debug, Clone, Eq)]
pub struct SelectedNamespace {
    pub idx: usize,
}

#[derive(Default, Clone, Debug)]
pub struct SelectedCommand {
    pub value: Command<'static>,
    pub current_idx: usize,
}

impl SelectedCommand {
    pub fn new(value: Command<'static>, current_idx: usize) -> Self {
        Self { value, current_idx }
    }
}
const DEFAULT_NAMESPACE: &str = "All";

impl State {
    pub fn new() -> State {
        let cfg = DefaultConfig::load().unwrap();
        let command_map = fs::load_from(cfg.command_file_path()).unwrap();
        let commands = Commands::init(command_map);
        let current_items = commands.as_list().sort_and_return();
        let selected = current_items.first();

        // namespaces
        let mut namespaces: Vec<String> = commands.as_map().keys().cloned().collect();
        namespaces.push(DEFAULT_NAMESPACE.to_string());
        namespaces.sort_by_key(|a| a.to_lowercase());

        Self {
            commands,
            current_items,
            selected_namespace: SelectedNamespace { idx: 0 },
            selected_command: SelectedCommand {
                value: selected,
                current_idx: 0,
            },
            namespaces,
            config: cfg,
        }
    }

    pub fn select(&mut self, idx: usize) {
        self.selected_command = SelectedCommand {
            value: self.current_items[idx].clone(),
            current_idx: idx,
        };
    }

    pub fn next_item(&mut self) -> SelectedCommand {
        let items = self.current_items.to_owned();
        let current = self.selected_command.current_idx;
        let next = (current + 1) % items.len();

        self.select(next);

        self.selected_command.to_owned()
    }

    pub fn previous_item(&mut self) -> SelectedCommand {
        let items = self.current_items.to_owned();
        let current = self.selected_command.current_idx;
        let previous = (current + items.len() - 1) % items.len();

        self.select(previous);

        self.selected_command.to_owned()
    }

    pub fn next_tab(&mut self) -> SelectedNamespace {
        let current = self.selected_namespace.idx;
        let items = &self.namespaces;
        let next = (current + 1) % items.len();
        let next_namespace = &self.namespaces[next];

        self.current_items = self.filter_command_by_namespace(next_namespace);
        self.selected_namespace = SelectedNamespace { idx: next };
        self.selected_command = SelectedCommand {
            current_idx: 0,
            value: self.current_items.first(),
        };

        self.selected_namespace.to_owned()
    }

    pub fn previous_tab(&mut self) -> SelectedNamespace {
        let current = self.selected_namespace.idx;
        let items = &self.namespaces;
        let previous = (current + items.len() - 1) % items.len();
        let previous_namespace = &self.namespaces[previous];

        self.current_items = self.filter_command_by_namespace(previous_namespace);
        self.selected_namespace = SelectedNamespace { idx: previous };
        self.selected_command = SelectedCommand {
            current_idx: 0,
            value: self.current_items.first(),
        };

        self.selected_namespace.to_owned()
    }

    pub fn execute(&self) {
        let command = &self.selected_command.value;
        debug!("Executing command: {:?}", command);
        command
            .exec(false, self.config.preferences().quiet_mode())
            .unwrap();
    }

    // TODO should persist the changes
    pub fn delete_command(&mut self) {
        let command = &self.selected_command.value;
        match self.commands.remove(command) {
            Ok(_) => {
                self.current_items = self.commands.as_list().sort_and_return();
                self.selected_command = SelectedCommand {
                    value: self.current_items.first(),
                    current_idx: 0,
                };
            }
            Err(e) => {
                error!("Error deleting command: {:?}", e);
            }
        }
    }

    fn filter_command_by_namespace(&self, namespace: &str) -> CommandVec<'static> {
        {
            if namespace == DEFAULT_NAMESPACE {
                self.commands.as_list()
            } else {
                self.commands
                    .get_namespace_content(namespace)
                    .cloned()
                    .unwrap_or_default()
            }
        }
        .sort_and_return()
    }
}
