use std::fmt;

use crate::gui::layouts::TerminalSize;

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

#[derive(Clone, Debug)]
pub struct UiState {
    pub view_mode: ViewMode,
    pub size: TerminalSize,
}

impl UiState {
    pub fn new(size: TerminalSize) -> UiState {
        Self {
            view_mode: ViewMode::Main,
            size,
        }
    }
}
