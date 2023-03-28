use super::{
    application_context::ApplicationContext, events::input_events::InputMessages, ui_state::UiState,
};
use crate::gui::layouts::select_ui;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{debug, error};
use parking_lot::Mutex;
use std::{
    io, panic,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::mpsc::Sender;
use tui::{backend::CrosstermBackend, Terminal};

pub struct TuiApplication {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub input_sx: Sender<InputMessages>,
    pub should_quit: Arc<AtomicBool>,
    pub ui_state: Arc<Mutex<UiState>>,
    context: Arc<Mutex<ApplicationContext<'static>>>,
}

impl TuiApplication {
    pub fn create(
        input_sx: Sender<InputMessages>,
        should_quit: Arc<AtomicBool>,
        ui_state: Arc<Mutex<UiState>>,
        context: Arc<Mutex<ApplicationContext<'static>>>,
    ) -> Result<TuiApplication> {
        Self::handle_panic();
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        Ok(TuiApplication {
            terminal,

            input_sx,
            should_quit,
            ui_state,
            context,
        })
    }

    pub async fn render(&mut self) -> Result<()> {
        while !self.should_quit.load(Ordering::SeqCst) {
            self.terminal
                .draw(|frame| select_ui(frame, &mut self.ui_state, &mut self.context))?;
            if crossterm::event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) {
                if let Event::Key(key) = event::read()? {
                    self.input_sx.send(InputMessages::KeyPress(key)).await.ok();
                }
            }
        }
        debug!("quiting tui app loop");
        Ok(())
    }

    fn handle_panic() {
        panic::set_hook(Box::new(|e| {
            eprintln!("{e}");
            error!("{e}")
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
        self.context.lock().execute_callback_command()?;
        Ok(())
    }
}

impl Drop for TuiApplication {
    fn drop(&mut self) {
        self.clear().expect("Cannot clear the the screen");
        self.callback()
            .expect("Cannot execute the selected command");
        debug!("shutting down the app");
    }
}
