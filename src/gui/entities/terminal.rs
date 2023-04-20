use super::contexts::{application_context::ApplicationContext, ui_context::UIContext};
use crate::gui::screens::Screen;
use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::debug;
use parking_lot::Mutex;
use std::{
    io::{self, Stdout},
    sync::Arc,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Rect,
    Terminal as TuiTerminal,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TerminalSize {
    Small,
    #[default]
    Medium,
    Large,
}

pub struct Terminal<B>
where
    B: Backend,
{
    tui_terminal: TuiTerminal<B>,
}

impl Terminal<CrosstermBackend<Stdout>> {
    pub fn new() -> Result<Terminal<CrosstermBackend<Stdout>>> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = TuiTerminal::new(backend)?;

        terminal.hide_cursor()?;

        Ok(Self {
            tui_terminal: terminal,
        })
    }

    pub fn size(&mut self) -> TerminalSize {
        self.tui_terminal.get_frame().size().as_terminal_size()
    }

    pub fn clear(&mut self) -> Result<()> {
        debug!("clearing the screen");
        disable_raw_mode()?;
        execute!(
            self.tui_terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;

        self.tui_terminal.show_cursor()?;

        Ok(())
    }

    pub fn draw(
        &mut self,
        ui_context: &mut Arc<Mutex<UIContext>>,
        app_context: &mut Arc<Mutex<ApplicationContext>>,
        screen: &mut dyn Screen<CrosstermBackend<Stdout>>,
    ) -> Result<()> {
        self.tui_terminal
            .draw(|frame| screen.render(frame, &mut app_context.lock(), &mut ui_context.lock()))?;
        Ok(())
    }
}

pub trait TerminalSizeExt {
    fn as_terminal_size(&self) -> TerminalSize;
}

impl TerminalSizeExt for Rect {
    fn as_terminal_size(&self) -> TerminalSize {
        let height = self.height;
        if height <= 20 {
            TerminalSize::Small
        } else if height <= 30 {
            TerminalSize::Medium
        } else {
            TerminalSize::Large
        }
    }
}
