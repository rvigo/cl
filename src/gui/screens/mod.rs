mod form_screen;
mod main_screen;
pub(super) mod widgets;

use super::entities::{
    contexts::{application_context::ApplicationContext, ui_context::UIContext},
    states::ui_state::ViewMode,
};
use crate::gui::screens::{form_screen::FormScreen, main_screen::MainScreen};
use std::collections::HashMap;
use tui::{backend::Backend, Frame};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ScreenSize {
    Small,
    #[default]
    Medium,
    Large,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ScreenType {
    Main,
    Form(Operation),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Operation {
    Insert,
    Edit,
}

impl From<ViewMode> for ScreenType {
    fn from(value: ViewMode) -> Self {
        match value {
            ViewMode::Main => ScreenType::Main,
            ViewMode::Insert => ScreenType::Form(Operation::Insert),
            ViewMode::Edit => ScreenType::Form(Operation::Edit),
        }
    }
}

impl From<ScreenType> for ViewMode {
    fn from(value: ScreenType) -> Self {
        match value {
            ScreenType::Main => ViewMode::Main,
            ScreenType::Form(op) => match op {
                Operation::Insert => ViewMode::Insert,
                Operation::Edit => ViewMode::Edit,
            },
        }
    }
}

/// Represents a Screen of `B` where `B` is a Tui `Backend`
pub trait Screen<B>
where
    B: Backend,
{
    fn set_screen_size(&mut self, screen_size: ScreenSize);

    fn get_screen_size(&self) -> ScreenSize;

    fn render(
        &mut self,
        frame: &mut Frame<B>,
        application_context: &mut ApplicationContext,
        ui_context: &mut UIContext,
    );
}

/// Screens aggregator
pub struct Screens<'a, B>
where
    B: Backend,
{
    screens: HashMap<ScreenType, Box<dyn Screen<B> + 'a>>,
}

impl<'a, B> Screens<'a, B>
where
    B: Backend,
{
    pub fn new<I>(size: I) -> Screens<'a, B>
    where
        I: Into<ScreenSize>,
    {
        let size = size.into();
        let main_screen = MainScreen::new(size.clone());
        let insert_screen = FormScreen::new(size.clone());
        let edit_screen = FormScreen::new(size);

        let mut screens = Self {
            screens: HashMap::new(),
        };

        screens.register(ScreenType::Main, main_screen);
        screens.register(ScreenType::Form(Operation::Insert), insert_screen);
        screens.register(ScreenType::Form(Operation::Edit), edit_screen);

        screens
    }

    pub fn register<S>(&mut self, screen_type: ScreenType, screen: S)
    where
        S: Screen<B> + 'a,
    {
        self.screens.insert(screen_type, Box::new(screen));
    }

    pub fn get_screen<I>(&mut self, screen_type: I) -> Option<&mut Box<dyn Screen<B> + 'a>>
    where
        I: Into<ScreenType>,
    {
        let st: ScreenType = screen_type.into();
        self.screens.get_mut(&st)
    }
}
