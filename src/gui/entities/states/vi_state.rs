use std::fmt::{Display, Error, Formatter};

#[derive(Default, Clone, Debug)]
pub enum ViMode {
    #[default]
    Normal,
    Insert,
}

impl Display for ViMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Normal => write!(f, "--NORMAL--"),
            Self::Insert => write!(f, "--INSERT--"),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct ViState {
    mode: ViMode,
    /// Keybindings enabled flag
    enabled: bool,
}

/// Represents the `Vi editor` state
impl ViState {
    pub fn init(enabled: bool) -> ViState {
        Self {
            mode: ViMode::Normal,
            enabled,
        }
    }

    pub fn change_mode_to(&mut self, mode: ViMode) {
        self.mode = mode
    }

    pub fn mode(&self) -> &ViMode {
        &self.mode
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}
