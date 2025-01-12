use cl_core::{fs, Command, CommandExec, Commands, Config, DefaultConfig};
use log::debug;

#[derive(Default)]
pub struct State {
    pub commands: Commands<'static>,
    pub selected_command: SelectedCommand,
}

#[derive(Default, Clone, Debug)]
pub struct SelectedCommand {
    pub value: Command<'static>,
    pub current_idx: usize,
}

impl State {
    pub fn new() -> State {
        let cfg = DefaultConfig::load().unwrap();
        let command_map = fs::load_from(cfg.command_file_path()).unwrap();
        let commands = Commands::init(command_map);

        let selected = commands.as_list()[0].clone();

        Self {
            commands,
            selected_command: SelectedCommand {
                value: selected,
                current_idx: 0,
            },
        }
    }

    pub fn select(&mut self, idx: usize) {
        self.selected_command.current_idx = idx;
        self.selected_command.value = self.commands.as_list()[idx].clone();
    }

    pub fn next(&mut self) -> SelectedCommand {
        let items = self.commands.as_list();
        let current = self.selected_command.current_idx;
        let next = (current + 1) % items.len();

        // TODO check if this method is correct
        self.select(next);

        SelectedCommand {
            value: items[next].clone(),
            current_idx: next,
        }
    }

    pub fn previous(&mut self) -> SelectedCommand {
        let items = self.commands.as_list();
        let current = self.selected_command.current_idx;
        let previous = (current + items.len() - 1) % items.len();

        self.select(previous);

        SelectedCommand {
            value: items[previous].clone(),
            current_idx: previous,
        }
    }

    pub fn execute(&self) {
        let command = &self.selected_command.value;
        debug!("Executing command: {:?}", command);
        command.exec(true, false).unwrap();
    }
}
