use crate::component::Renderable;
use std::time::{Duration, Instant};
use tui::layout::Alignment::Center;
use tui::layout::Rect;
use tui::style::Color::{Black, Green};
use tui::style::Style;
use tui::widgets::Paragraph;
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

impl Renderable for ClipboardStatus {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        if self.copied {
            let paragraph = Paragraph::new("copied")
                .alignment(Center)
                .style(Style::default().fg(Black).bg(Green));

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
