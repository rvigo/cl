use crate::component::Renderable;
use crate::screen::theme::Theme;
use std::time::{Duration, Instant};
use tui::layout::Alignment::Center;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Block, Paragraph};
use tui::Frame;

#[derive(Debug)]
pub struct ClipboardStatus {
    copied: bool,
    state: ClipboardState,
}

impl ClipboardStatus {
    pub fn new() -> Self {
        Self {
            copied: false,
            state: ClipboardState::default(),
        }
    }

    pub fn start_counter(&mut self) {
        self.copied = true;
        self.state.start();
    }

    pub fn check_if_need_to_stop(&mut self) {
        self.state.check();
        if !self.state.running {
            self.copied = false;
        }
    }
}

impl Default for ClipboardStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for ClipboardStatus {
    fn render(&mut self, frame: &mut Frame, area: Rect, theme: &Theme) {
        let theme = theme.to_owned();
        if self.copied {
            let paragraph = Paragraph::new("Command copied to the clipboard")
                .alignment(Center)
                .style(
                    Style::default()
                        .fg(theme.highlight_color.into())
                        .bg(theme.background_color.into()),
                )
                .block(Block::bordered());

            frame.render_widget(paragraph, area);

            self.check_if_need_to_stop()
        } else {
            let paragraph = Paragraph::new("Press Y to copy the command")
                .alignment(Center)
                .style(
                    Style::default()
                        .fg(theme.text_color.into())
                        .bg(theme.background_color.into()),
                )
                .block(Block::bordered());

            frame.render_widget(paragraph, area);

            self.check_if_need_to_stop()
        }
    }
}

#[derive(Debug)]
struct ClipboardState {
    start_instant: Option<Instant>,
    duration: u64,
    running: bool,
}

impl ClipboardState {
    fn start(&mut self) {
        self.start_instant = Some(Instant::now());
        self.running = true;
    }

    fn check(&mut self) {
        if let Some(instant) = self.start_instant {
            if instant.elapsed().as_secs() == Duration::new(self.duration, 0).as_secs() {
                self.start_instant = None;
                self.running = false;
            }
        }
    }
}

impl Default for ClipboardState {
    fn default() -> Self {
        Self {
            start_instant: Default::default(),
            duration: 3,
            running: false,
        }
    }
}
