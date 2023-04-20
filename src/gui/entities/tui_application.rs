use super::{
    contexts::{application_context::ApplicationContext, ui_context::UIContext},
    events::input_events::InputMessages,
    terminal::Terminal,
};
use crate::gui::screens::Screens;
use anyhow::Result;
use crossterm::event::{self, Event};
use log::{debug, error};
use parking_lot::Mutex;
use std::{
    io::Stdout,
    panic,
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
    screens: Screens<'a, CrosstermBackend<Stdout>>,
}

impl<'a> TuiApplication<'a> {
    pub fn create(
        input_sx: Sender<InputMessages>,
        should_quit: Arc<AtomicBool>,
        ui_context: Arc<Mutex<UIContext<'a>>>,
        context: Arc<Mutex<ApplicationContext>>,
        terminal: Terminal<CrosstermBackend<Stdout>>,
        screens: Screens<'a, CrosstermBackend<Stdout>>,
    ) -> Result<TuiApplication<'a>> {
        Self::handle_panic();
        Ok(TuiApplication {
            terminal,
            input_sx,
            should_quit,
            ui_context,
            context,
            screens,
        })
    }

    pub async fn render(&mut self) -> Result<()> {
        while !self.should_quit.load(Ordering::SeqCst) {
            let view_mode = self.ui_context.clone().lock().view_mode(); // TODO can this be improved?
            if let Some(screen) = self.screens.get_screen(view_mode) {
                self.terminal
                    .draw(&mut self.ui_context, &mut self.context, &mut **screen)?;
                if event::poll(Duration::from_millis(0)).unwrap_or(false) {
                    if let Ok(event) = event::read() {
                        if let Event::Key(key) = event {
                            self.input_sx.send(InputMessages::KeyPress(key)).await.ok();
                        } else if let Event::Resize(_, _) = event {
                            screen.set_screen_size(self.terminal.size().into())
                        }
                    }
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
        self.terminal.clear()
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
