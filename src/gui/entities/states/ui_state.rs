use crate::gui::{entities::terminal::TerminalSize, screens::ScreenSize};
use std::fmt;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ViewMode {
    #[default]
    Main,
    Insert,
    Edit,
}

impl fmt::Display for ViewMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ViewMode::Main => write!(f, "Main"),
            ViewMode::Insert => write!(f, "Insert"),
            ViewMode::Edit => write!(f, "Edit"),
        }
    }
}

impl From<&TerminalSize> for &ScreenSize {
    fn from(value: &TerminalSize) -> Self {
        match value {
            TerminalSize::Small => &ScreenSize::Small,
            TerminalSize::Medium => &ScreenSize::Medium,
            TerminalSize::Large => &ScreenSize::Large,
        }
    }
}

impl From<TerminalSize> for ScreenSize {
    fn from(value: TerminalSize) -> Self {
        match value {
            TerminalSize::Small => ScreenSize::Small,
            TerminalSize::Medium => ScreenSize::Medium,
            TerminalSize::Large => ScreenSize::Large,
        }
    }
}

#[derive(Debug)]
pub struct UiState {
    view_mode: ViewMode,
    screen_size: ScreenSize,
    show_popup: bool,
    show_help: bool,
    querybox_focus: bool,
}

impl UiState {
    pub fn new(screen_size: ScreenSize) -> UiState {
        Self {
            view_mode: ViewMode::Main,
            screen_size,
            show_popup: false,
            show_help: false,
            querybox_focus: false,
        }
    }

    pub fn querybox_focus(&self) -> bool {
        self.querybox_focus
    }

    pub fn set_querybox_focus(&mut self, focus: bool) {
        self.querybox_focus = focus
    }

    pub fn view_mode(&self) -> ViewMode {
        self.view_mode.to_owned()
    }

    pub fn set_view_mode(&mut self, view_mode: ViewMode) {
        self.view_mode = view_mode
    }

    pub fn screen_size(&self) -> ScreenSize {
        self.screen_size.to_owned()
    }

    pub fn set_screen_size(&mut self, screen_size: ScreenSize) {
        self.screen_size = screen_size
    }

    pub fn show_popup(&self) -> bool {
        self.show_popup
    }

    pub fn set_show_popup(&mut self, should_show: bool) {
        self.show_popup = should_show
    }

    pub fn show_help(&self) -> bool {
        self.show_help
    }

    pub fn set_show_help(&mut self, should_show: bool) {
        self.show_help = should_show
    }
}
