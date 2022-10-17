use std::fmt;

pub enum ViewMode {
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
