mod form_screen;
mod main_screen;

use super::entities::{
    contexts::{application_context::ApplicationContext, ui_context::UIContext},
    states::ui_state::ViewMode,
};
use crate::{
    screens::{form_screen::FormScreen, main_screen::MainScreen},
    widgets::{base_widget::BaseWidget, statusbar::StatusBarItem},
};
use std::collections::HashMap;
use tui::Frame;

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

/// Represents a Screen
pub trait Screen {
    fn set_screen_size(&mut self, screen_size: ScreenSize);

    fn get_screen_size(&self) -> ScreenSize;

    fn render(
        &mut self,
        frame: &mut Frame,
        application_context: &mut ApplicationContext,
        ui_context: &mut UIContext,
    );
}

/// Screens aggregator
pub struct Screens<'a> {
    screens: HashMap<ScreenType, Box<dyn Screen + 'a>>,
}

impl<'a> Screens<'a> {
    pub fn new<I>(size: I) -> Screens<'a>
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
        S: Screen + 'a,
    {
        self.screens.insert(screen_type, Box::new(screen));
    }

    pub fn get_screen<I>(&mut self, screen_type: I) -> Option<&mut Box<dyn Screen + 'a>>
    where
        I: Into<ScreenType>,
    {
        let st: ScreenType = screen_type.into();
        self.screens.get_mut(&st)
    }
}

/// Extension for `Screen`
pub trait ScreenExt: Screen {
    fn render_base<F, H>(
        &self,
        frame: &mut Frame,
        left_statusbar_item: Option<&F>,
        right_statusbar_item: Option<H>,
    ) where
        F: StatusBarItem,
        H: StatusBarItem,
    {
        let terminal_size = self.get_screen_size();
        let var_name = BaseWidget::new(&terminal_size, left_statusbar_item, right_statusbar_item);
        let base_widget = var_name;

        frame.render_widget(base_widget, frame.size());
    }
}

impl<T> ScreenExt for T where T: Screen {}
