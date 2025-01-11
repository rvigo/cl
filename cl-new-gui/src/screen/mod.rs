use crate::textbox::TextBox;
use tui::Frame;

#[derive(Default)]
pub enum ActiveScreen {
    #[default]
    Main,
}

#[derive(Default)]
pub struct Screens {
    pub active_screen: ActiveScreen,
    pub main: MainScreen,
}

impl Screens {
    pub fn new() -> Screens {
        Self {
            ..Default::default()
        }
    }

    pub fn get_active_screen(&self) -> &dyn Screen {
        match self.active_screen {
            ActiveScreen::Main => &self.main,
        }
    }
}

pub trait Screen {
    fn new(component: TextBox) -> Self
    where
        Self: Sized;

    fn render(&self, frame: &mut Frame);
}

#[derive(Default)]
pub struct MainScreen {
    pub component: TextBox,
}

impl Screen for MainScreen {
    fn new(component: TextBox) -> Self {
        Self { component }
    }

    fn render(&self, frame: &mut Frame) {
        self.component.render(frame)
    }
}

pub trait Component {
    fn render(&self, frame: &mut Frame);
}
