mod main_screen;

use crate::observer::ComponentPublisher;
use crate::screen::main_screen::MainScreen;
use tui::Frame;

#[derive(Default)]
pub enum ActiveScreen {
    #[default]
    Main,
}

pub struct Screens {
    pub active_screen: ActiveScreen,
    pub main: MainScreen,
}

impl Screens {
    pub fn new() -> Screens {
        Self {
            active_screen: ActiveScreen::Main,
            main: MainScreen::new(),
        }
    }

    pub fn get_active_screen(&self) -> &dyn Screen {
        match self.active_screen {
            ActiveScreen::Main => &self.main,
        }
    }

    pub fn get_active_screen_mut(&mut self) -> &mut dyn Screen {
        match self.active_screen {
            ActiveScreen::Main => &mut self.main,
        }
    }
}

pub trait Screen {
    fn new() -> Self
    where
        Self: Sized;

    fn render(&mut self, frame: &mut Frame);

    fn get_publisher(&mut self) -> &mut ComponentPublisher;
}
