mod form_screen;
mod main_screen;

use super::entities::contexts::{application_context::ApplicationContext, ui::UI};
use crate::{
    entities::{terminal::TerminalSizeExt, view_mode::ViewMode},
    register, render,
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
        ui_context: &mut UI,
    );
}

type ScreenRegistrar<'screen> = HashMap<ScreenType, Box<dyn Screen + 'screen>>;

/// Screens aggregator
pub struct Screens<'screen> {
    screens: ScreenRegistrar<'screen>,
}

impl<'screen> Screens<'screen> {
    pub fn new() -> Screens<'screen> {
        let mut screens: ScreenRegistrar<'screen> = ScreenRegistrar::new();

        register!(
            screens,
            ScreenType::Main => Box::new(MainScreen),
            ScreenType::Form(Operation::Insert) => Box::new(FormScreen),
            ScreenType::Form(Operation::Edit) => Box::new(FormScreen)
        );

        Self { screens }
    }

    pub fn get_screen<I>(&mut self, screen_type: I) -> Option<&mut Box<dyn Screen + 'screen>>
    where
        I: Into<ScreenType>,
    {
        let st: ScreenType = screen_type.into();
        self.screens.get_mut(&st)
    }
}

impl Default for Screens<'_> {
    fn default() -> Self {
        Self::new()
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
        let area = frame.size();

        let terminal_size = area.as_terminal_size();
        let base_widget = BaseWidget::new(
            &terminal_size,
            left_statusbar_item,
            center_statusbar_item,
            right_statusbar_item,
        );

        render!(frame, { base_widget, area });
    }
}

impl<T> ScreenExt for T where T: Screen {}
