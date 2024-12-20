use std::time::{Duration, Instant};

#[derive(Default)]
pub struct ClipboardState {
    yanked: bool,
    timer_counter: ClipboardTimerCount,
}

impl ClipboardState {
    pub fn yanked(&self) -> bool {
        self.yanked
    }

    pub fn start_counter(&mut self) {
        self.yanked = true;
        self.timer_counter.start();
    }

    pub fn check_if_need_to_stop(&mut self) {
        self.timer_counter.check();
        if !self.timer_counter.running {
            self.yanked = false;
        }
    }
}

struct ClipboardTimerCount {
    start_instant: Option<Instant>,
    duration: u64,
    running: bool,
}

impl ClipboardTimerCount {
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

impl Default for ClipboardTimerCount {
    fn default() -> Self {
        Self {
            start_instant: Default::default(),
            duration: 3,
            running: false,
        }
    }
}
