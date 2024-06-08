use std::time::{Duration, Instant};

pub struct ClipboardState {
    yanked: bool,
    start_instant: Option<Instant>,
    duration: u64,
}

impl ClipboardState {
    pub fn yanked(&self) -> bool {
        self.yanked
    }

    pub fn start(&mut self) {
        self.yanked = true;
        self.start_instant = Some(Instant::now());
    }

    pub fn check(&mut self) {
        if let Some(instant) = self.start_instant {
            if instant.elapsed().as_secs() == Duration::new(self.duration, 0).as_secs() {
                self.stop()
            }
        }
    }

    fn stop(&mut self) {
        self.yanked = false;
        self.start_instant = None;
    }
}

impl Default for ClipboardState {
    fn default() -> Self {
        Self {
            yanked: Default::default(),
            start_instant: Default::default(),
            duration: 3,
        }
    }
}
