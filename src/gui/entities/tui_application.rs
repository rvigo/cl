use crate::{
    gui::{
        entities::application_context::ApplicationContext,
        key_handlers,
        layouts::{get_terminal_size, select_ui},
    },
    resources::{config::Config, file_service::FileService},
};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::debug;
use std::{io, panic};
use tui::{backend::CrosstermBackend, Terminal};

pub struct TuiApplication<'a> {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub context: ApplicationContext<'a>,
    config: Config,
}

impl<'a> TuiApplication<'a> {
    pub fn create() -> Result<TuiApplication<'a>> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        let size = get_terminal_size(&terminal.get_frame());
        let config = Config::load()?;
        let file_service = FileService::new(config.get_command_file_path()?);
        let commands = file_service.load_commands_from_file()?;
        let context = ApplicationContext::init(commands, size, file_service);

        Ok(TuiApplication {
            terminal,
            context,
            config,
        })
    }

    pub fn render(&mut self) -> Result<()> {
        self.handle_panic();
        loop {
            self.terminal
                .draw(|frame| select_ui(frame, &mut self.context))?;
            if let Event::Key(key) = event::read()? {
                key_handlers::handle(key, &mut self.context);
                if self.context.should_quit() {
                    return Ok(());
                }
            }
        }
    }

    fn handle_panic(&self) {
        panic::set_hook(Box::new(|e| {
            eprintln!("{e}");
            log::error!("{e}")
        }));
    }

    fn clear(&mut self) -> Result<()> {
        debug!("clearing the screen");
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        self.terminal.show_cursor()?;

        Ok(())
    }

    fn callback(&self) -> Result<()> {
        debug!("executing the callback command");
        self.context
            .execute_callback_command(self.config.get_default_quiet_mode())
    }
}

impl<'a> Drop for TuiApplication<'a> {
    fn drop(&mut self) {
        self.clear().expect("Cannot clear the the screen");
        self.callback()
            .expect("Cannot execute the selected command");
        debug!("shutting down the app");
    }
}
