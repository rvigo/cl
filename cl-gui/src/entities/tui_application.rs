use super::{
    contexts::{application_context::ApplicationContext, ui_context::UIContext},
    events::input_events::InputMessages,
    states::ui_state::ViewMode,
    terminal::Terminal,
};
use crate::screens::Screens;
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

pub struct TuiApplication<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    input_sx: Sender<InputMessages>,
    should_quit: Arc<AtomicBool>,
    ui_context: Arc<Mutex<UIContext<'a>>>,
    context: Arc<Mutex<ApplicationContext>>,
    screens: Screens<'a>,
}

impl<'a> TuiApplication<'a> {
    pub fn create(
        input_sx: Sender<InputMessages>,
        should_quit: Arc<AtomicBool>,
        ui_context: Arc<Mutex<UIContext<'a>>>,
        context: Arc<Mutex<ApplicationContext>>,
        terminal: Terminal<CrosstermBackend<Stdout>>,
        screens: Screens<'a>,
    ) -> Result<TuiApplication<'a>> {
        let tui = TuiApplication {
            terminal,
            input_sx,
            should_quit,
            ui_context,
            context,
            screens,
        };

        Ok(tui)
    }

    pub async fn render(&mut self) -> Result<()> {
        while !self.should_quit.load(Ordering::SeqCst) {
            let view_mode = self.get_current_screen_type();

            if let Some(screen) = self.screens.get_screen(view_mode) {
                self.terminal
                    .draw(&mut self.ui_context, &mut self.context, &mut **screen)?;

                if event::poll(Duration::from_millis(50))? {
                    if let Ok(Event::Key(key)) = event::read() {
                        self.input_sx.send(InputMessages::KeyPress(key)).await?;
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

    fn get_current_screen_type(&self) -> ViewMode {
        let ui_context = self.ui_context.lock();
        ui_context.view_mode()
    }

    fn callback(&self) -> Result<()> {
        self.context.lock().execute_callback_command()
    }
}
