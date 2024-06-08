mod form_screen;
mod main_screen;

use crate::context::{Application, UI};
use crate::{
    register, render,
    screen::{form_screen::FormScreen, main_screen::MainScreen},
    terminal::TerminalSizeExt,
    widget::{statusbar::StatusBarItem, BaseWidget},
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
    fn render(&self, frame: &mut Frame, application_context: &mut Application, ui_context: &mut UI);
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

    pub fn get_screen_by_type<I>(&mut self, screen_type: I) -> Option<&(dyn Screen + 'screen)>
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

/// Extension for `Screen`
pub trait ScreenExt {
    fn render_base(
        &self,
        frame: &mut Frame,
        left_statusbar_item: Option<impl StatusBarItem>,
        center_statusbar_item: Option<impl StatusBarItem>,
        right_statusbar_item: Option<impl StatusBarItem>,
    ) {
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
