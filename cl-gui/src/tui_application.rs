use crate::{
    context::{Application, UI},
    event::InputEvent,
    screen::Screens,
    Terminal, ViewMode,
};
use anyhow::{Context, Result};
use crossterm::event::{self, Event};
use log::debug;
use parking_lot::Mutex;
use std::{
    io::Stdout,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::mpsc::Sender;
use tui::backend::CrosstermBackend;

pub struct TuiApplication<'tui> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    input_sx: Sender<InputEvent>,
    should_quit: Arc<AtomicBool>,
    ui: Arc<Mutex<UI<'tui>>>,
    context: Arc<Mutex<Application>>,
    screens: Screens<'tui>,
}

impl<'tui> TuiApplication<'tui> {
    pub fn create(
        input_sx: Sender<InputEvent>,
        should_quit: Arc<AtomicBool>,
        ui: Arc<Mutex<UI<'tui>>>,
        context: Arc<Mutex<Application>>,
        terminal: Terminal<CrosstermBackend<Stdout>>,
        screens: Screens<'tui>,
    ) -> Result<TuiApplication<'tui>> {
        let tui = TuiApplication {
            terminal,
            input_sx,
            should_quit,
            ui,
            context,
            screens,
        };

        Ok(tui)
    }

    pub async fn render(&mut self) -> Result<()> {
        while !self.should_quit.load(Ordering::SeqCst) {
            let view_mode = self.current_view_mode();

            if let Some(screen) = self.screens.get_screen_by_type(view_mode) {
                self.terminal
                    .draw(&mut self.ui.lock(), &mut self.context.lock(), screen)?;

                if event::poll(Duration::from_millis(50))? {
                    if let Ok(Event::Key(key)) = event::read() {
                        self.input_sx.send(InputEvent::KeyPress(key)).await?;
                    }
                }
            }
        }

        debug!("quiting tui app loop");
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<()> {
        debug!("shutting down the app");
        self.terminal
            .restore()
            .context("Cannot clear the the screen")
            .and_then(|_| {
                self.callback()
                    .context("Cannot execute the selected command")
            })
    }

    fn current_view_mode(&self) -> ViewMode {
        self.ui.lock().view_mode()
    }

    fn callback(&self) -> Result<()> {
        self.context.lock().execute_callback()
    }
}
