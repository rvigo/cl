pub mod dialog_factory;

mod form_screen;
mod main_screen;
mod observer;

use crate::{
    context::{Application, UI},
    ViewMode,
};
use form_screen::FormScreen;
use main_screen::MainScreen;
use tui::Frame;

/// Represents a Screen
pub trait Screen<'s> {
    fn render(&mut self, frame: &mut Frame, application: &mut Application<'s>, ui: &mut UI<'s>);
}

/// Screens aggregator
pub struct Screens<'s> {
    main: Box<dyn Screen<'s> + 's>,
    edit: Box<dyn Screen<'s> + 's>,
    insert: Box<dyn Screen<'s> + 's>,
}

impl<'s> Screens<'s> {
    pub fn new() -> Screens<'s> {
        Self {
            main: Box::new(MainScreen::new()),
            edit: Box::new(FormScreen),
            insert: Box::new(FormScreen),
        }
    }

    pub fn get_screen_mut<I>(&mut self, screen_type: I) -> Option<&mut Box<dyn Screen<'s> + 's>>
    where
        I: Into<ScreenType>,
    {
        let st: ScreenType = screen_type.into();
        match st {
            ScreenType::Main => Some(&mut self.main),
            ScreenType::Form(Operation::Edit) => Some(&mut self.edit),
            ScreenType::Form(Operation::Insert) => Some(&mut self.insert),
        }
    }
}

impl Default for Screens<'_> {
    fn default() -> Self {
        Self::new()
    }
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
