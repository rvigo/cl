use std::{
    fmt,
    sync::{atomic::AtomicBool, Arc},
};

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
    pub show_popup: Arc<AtomicBool>,
    pub show_help: Arc<AtomicBool>,
    pub query_box_active: Arc<AtomicBool>,
}

impl UiState {
    pub fn new(size: TerminalSize) -> UiState {
        Self {
            view_mode: ViewMode::Main,
            size,
            show_popup: Arc::new(AtomicBool::new(false)),
            show_help: Arc::new(AtomicBool::new(false)),
            query_box_active: Arc::new(AtomicBool::new(false)),
        }
    }
}
