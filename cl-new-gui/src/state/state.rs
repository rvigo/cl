use cl_core::{fs, Command, CommandExec, CommandVec, Commands, Config, DefaultConfig};
use log::debug;

#[derive(Default)]
pub struct State {
    pub commands: Commands<'static>,
    pub current_items: CommandVec<'static>,
    pub selected_command: SelectedCommand,
    pub selected_namespace: SelectedNamespace,
    pub namespaces: Vec<String>,
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

impl State {
    pub fn new() -> State {
        let cfg = DefaultConfig::load().unwrap();
        let command_map = fs::load_from(cfg.command_file_path()).unwrap();
        let commands = Commands::init(command_map);
        let selected = commands.as_list()[0].clone();
        let current_items = commands
            .get_namespace_content(&selected.namespace)
            .cloned()
            .unwrap_or_default();
        let namespaces = commands.commands.keys().cloned().collect();

        Self {
            commands,
            current_items,
            selected_namespace: SelectedNamespace { idx: 0 },
            selected_command: SelectedCommand {
                value: selected,
                current_idx: 0,
            },
            namespaces,
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
        let target_namespace = self.namespaces[next].to_owned();

        let filtered = self
            .commands
            .get_namespace_content(&target_namespace)
            .cloned()
            .unwrap_or_default();

        self.current_items = filtered;
        self.selected_namespace = SelectedNamespace { idx: next };

        self.selected_namespace.to_owned()
    }

    pub fn previous_tab(&mut self) -> SelectedNamespace {
        let current = self.selected_namespace.idx;
        let items = &self.namespaces;
        let previous = (current + items.len() - 1) % items.len();
        let target_namespace = self.namespaces[previous].to_owned();

        let filtered = self
            .commands
            .get_namespace_content(&target_namespace)
            .cloned()
            .unwrap_or_default();

        self.current_items = filtered;
        self.selected_namespace = SelectedNamespace { idx: previous };

        self.selected_namespace.to_owned()
    }
    
    pub fn execute(&self) {
        let command = &self.selected_command.value;
        debug!("Executing command: {:?}", command);
        command.exec(true, false).unwrap();
    }
}
