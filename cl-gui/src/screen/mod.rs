mod form_screen;
mod main_screen;

use crate::context::{Application, UI};
use crate::{
    register,
    screen::{form_screen::FormScreen, main_screen::MainScreen},
    ViewMode,
};
use std::collections::HashMap;
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
    fn render(&self, frame: &mut Frame, application: &mut Application, ui: &mut UI);
}

type ScreenRegistrar<'screen> = HashMap<ScreenType, &'screen (dyn Screen + 'screen)>;

/// Screens aggregator
pub struct Screens<'screen> {
    screens: ScreenRegistrar<'screen>,
}

impl<'screen> Screens<'screen> {
    pub fn new() -> Screens<'screen> {
        let mut screens: ScreenRegistrar<'screen> = ScreenRegistrar::new();

        register!(
            screens,
            ScreenType::Main => &MainScreen,
            ScreenType::Form(Operation::Insert) => &FormScreen,
            ScreenType::Form(Operation::Edit) => &FormScreen
        );

        Self { screens }
    }

    pub fn get_screen<I>(&mut self, screen_type: I) -> Option<&(dyn Screen + 'screen)>
    where
        I: Into<ScreenType>,
    {
        let st: ScreenType = screen_type.into();
        self.screens.get(&st).copied()
    }
}

impl Default for Screens<'_> {
    fn default() -> Self {
        Self::new()
    }
}
