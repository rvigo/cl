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

#[derive(Debug)]
pub struct UiState {
    view_mode: ViewMode,
    show_popup: bool,
    show_help: bool,
}

impl UiState {
    pub fn new() -> UiState {
        Self {
            view_mode: ViewMode::Main,
            show_popup: false,
            show_help: false,
        }
    }

    pub fn view_mode(&self) -> ViewMode {
        self.view_mode.to_owned()
    }

    pub fn set_view_mode(&mut self, view_mode: ViewMode) {
        self.view_mode = view_mode
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
