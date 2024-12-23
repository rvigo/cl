pub mod dialog_factory;
mod form_screen;
mod main_screen;
mod observer;

use crate::{
    context::{Application, UI},
    register, ViewMode,
};
use form_screen::FormScreen;
use main_screen::MainScreen;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use tui::Frame;

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

/// Represents a Screen
pub trait Screen {
    fn render(&mut self, frame: &mut Frame, application: &mut Application, ui: &mut UI);
}

type ScreenRegistrar<'screen> = HashMap<ScreenType, BoxedScreen>;

pub struct BoxedScreen(Box<dyn Screen>);

impl BoxedScreen {
    pub fn new(screen: impl Screen + 'static) -> Self {
        Self(Box::new(screen))
    }
}

impl Deref for BoxedScreen {
    type Target = dyn Screen;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl DerefMut for BoxedScreen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

/// Screens aggregator
pub struct Screens {
    screens: ScreenRegistrar<'static>,
}

impl Screens {
    pub fn new() -> Screens {
        let mut screens: ScreenRegistrar = ScreenRegistrar::new();

        register!(
            screens,
            ScreenType::Main => BoxedScreen::new(MainScreen::new()),
            ScreenType::Form(Operation::Insert) => BoxedScreen::new(FormScreen),
            ScreenType::Form(Operation::Edit) => BoxedScreen::new(FormScreen)
        );

        Self { screens }
    }

    pub fn get_screen_mut<I>(&mut self, screen_type: I) -> Option<&mut BoxedScreen>
    where
        I: Into<ScreenType>,
    {
        let st: ScreenType = screen_type.into();
        self.screens.get_mut(&st)
    }
}

impl Default for Screens {
    fn default() -> Self {
        Self::new()
    }
}
