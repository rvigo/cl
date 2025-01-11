use cl_core::{fs, Command, Commands, Config, DefaultConfig};

#[derive(Default)]
pub struct State {
    pub commands: Commands<'static>,
    pub selected: Option<usize>,
}

impl State {
    pub fn new() -> State {
        let cfg = DefaultConfig::load().unwrap();
        let command_map = fs::load_from(cfg.command_file_path()).unwrap();
        let commands = Commands::init(command_map);

        Self {
            commands,
            selected: None,
        }
    }

    pub fn select(&mut self, idx: usize) {
        self.selected = Some(idx)
    }

    pub fn next(&mut self) -> Command<'static>{
        let items = self.commands.as_list();
        let current = self.selected.unwrap_or(0);
        let next = (current + 1) % items.len();

        self.select(next);
        items[next].clone()
    }

    pub fn previous(&mut self) -> Command<'static >{

        let items = self.commands.as_list();
        let current = self.selected.unwrap_or(0);
        let previous = (current + items.len() - 1) % items.len();

        self.select(previous);
        
        items[previous].clone()
    }
}
