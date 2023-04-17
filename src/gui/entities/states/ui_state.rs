pub use crate::gui::layouts::TerminalSize;
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

#[derive(Clone, Debug)]
pub struct UiState {
    view_mode: ViewMode,
    terminal_size: TerminalSize,
    show_popup: bool,
    show_help: bool,
    querybox_focus: bool,
}

impl UiState {
    pub fn new(size: TerminalSize) -> UiState {
        Self {
            view_mode: ViewMode::Main,
            terminal_size: size,
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

    pub fn terminal_size(&self) -> &TerminalSize {
        &self.terminal_size
    }

    pub fn set_terminal_size(&mut self, terminal_size: TerminalSize) {
        self.terminal_size = terminal_size
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
