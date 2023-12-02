mod form_screen;
mod main_screen;

use super::entities::{
    contexts::{application_context::ApplicationContext, ui_context::UIContext},
    states::ui_state::ViewMode,
};
use crate::{
    entities::terminal::TerminalSizeExt,
    screens::{form_screen::FormScreen, main_screen::MainScreen},
    widgets::{base_widget::BaseWidget, statusbar::StatusBarItem},
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
    pub fn new() -> Screens<'a> {
        let main_screen = MainScreen;
        let insert_screen = FormScreen;
        let edit_screen = FormScreen;

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
pub trait ScreenExt {
    fn render_base<L, C, R>(
        &self,
        frame: &mut Frame,
        left_statusbar_item: Option<&L>,
        center_statusbar_item: Option<R>,
        right_statusbar_item: Option<C>,
    ) where
        L: StatusBarItem,
        C: StatusBarItem,
        R: StatusBarItem,
    {
        let size = frame.size();

        let terminal_size = size.as_terminal_size();
        let base_widget = BaseWidget::new(
            &terminal_size,
            left_statusbar_item,
            center_statusbar_item,
            right_statusbar_item,
        );

        frame.render_widget(base_widget, size);
    }
}

impl<T> ScreenExt for T where T: Screen {}
