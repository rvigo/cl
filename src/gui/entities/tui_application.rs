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
    time::Duration,
};
use tokio::sync::mpsc::Sender;
use tui::{backend::CrosstermBackend, Terminal};

use super::{
    contexts::{application_context::ApplicationContext, ui_context::UIContext},
    events::input_events::InputMessages,
};

pub struct TuiApplication<'a> {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    input_sx: Sender<InputMessages>,
    should_quit: Arc<AtomicBool>,
    ui_context: Arc<Mutex<UIContext<'a>>>,
    context: Arc<Mutex<ApplicationContext>>,
}

impl<'a> TuiApplication<'a> {
    pub fn create(
        input_sx: Sender<InputMessages>,
        should_quit: Arc<AtomicBool>,
        ui_context: Arc<Mutex<UIContext<'a>>>,
        context: Arc<Mutex<ApplicationContext>>,
    ) -> Result<TuiApplication<'a>> {
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
            ui_context,
            context,
        })
    }

    pub async fn render(&mut self) -> Result<()> {
        while !self.should_quit.load(Ordering::SeqCst) {
            self.terminal
                .draw(|frame| select_ui(frame, &mut self.ui_context, &mut self.context))?;
            if event::poll(Duration::from_millis(100)).unwrap_or(false) {
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
        self.context.lock().execute_callback_command()?;
        Ok(())
    }
}

impl Drop for TuiApplication<'_> {
    fn drop(&mut self) {
        self.clear().expect("Cannot clear the the screen");
        self.callback()
            .expect("Cannot execute the selected command");
        debug!("shutting down the app");
    }
}
