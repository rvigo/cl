use crate::{
    commands::Commands,
    gui::{contexts::state::State, key_handler::handle, layouts::selector::select_ui},
};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::info;
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

pub struct AppContext {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub state: State,
}

impl AppContext {
    pub fn create(commands: Commands) -> Result<AppContext> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        Ok(AppContext {
            terminal,
            state: State::init(commands.clone()),
        })
    }

    pub fn render(&mut self) -> Result<()> {
        info!("starting the render process");
        loop {
            self.terminal
                .draw(|frame| select_ui(frame, &mut self.state))?;
            if let Event::Key(key) = event::read()? {
                handle(key, self.state.get_mut_ref());
                if self.state.should_quit {
                    info!("ending app");
                    return Ok(());
                }
            }
        }
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
        self.state.execute_callback_command()?;
        Ok(())
    }
}
