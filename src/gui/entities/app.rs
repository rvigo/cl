use crate::{
    commands::Commands,
    gui::{entities::state::State, key_handler::KeyHandler, layouts::selector::select_ui},
    resources::{app_configuration::AppConfiguration, file_service::CommandFileService},
};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, panic};
use tui::{backend::CrosstermBackend, Terminal};

pub struct AppContext {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub state: State,
    pub key_handler: KeyHandler,
}

impl AppContext {
    pub fn create() -> Result<AppContext> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        let config = AppConfiguration::init()?;
        let command_file_service = CommandFileService::init(config.command_file_path());

        let command_items = command_file_service.load_commands_from_file()?;
        let commands = Commands::init(command_items);

        let key_handler = KeyHandler::new(command_file_service);

        Ok(AppContext {
            terminal,
            state: State::init(commands),
            key_handler,
        })
    }

    pub fn render(&mut self) -> Result<()> {
        self.handle_panic();
        loop {
            self.terminal
                .draw(|frame| select_ui(frame, &mut self.state))?;
            if let Event::Key(key) = event::read()? {
                self.key_handler.handle(key, &mut self.state);
                if self.state.should_quit {
                    return Ok(());
                }
            }
        }
    }

    fn handle_panic(&self) {
        panic::set_hook(Box::new(|e| {
            eprintln!("{}", e);
        }));
    }

    pub fn clear(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        self.terminal.show_cursor()?;

        Ok(())
    }

    pub fn callback_command(&self) -> Result<()> {
        self.state.execute_callback_command()
    }
}
