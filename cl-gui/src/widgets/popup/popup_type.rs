#[derive(Default, Debug, Clone)]
pub enum PopupType {
    #[default]
    Error,
    Warning,
    Help,
}

impl ToString for PopupType {
    fn to_string(&self) -> String {
        match self {
            PopupType::Error => "Error".to_owned(),
            PopupType::Warning => "Warning".to_owned(),
            PopupType::Help => "Help".to_owned(),
        }
    }
}
