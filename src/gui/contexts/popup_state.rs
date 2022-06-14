use std::fmt;

#[derive(Debug, Clone)]
pub enum MessageType {
    None,
    Error,
    Confirmation,
}

#[derive(Debug, Clone)]
pub enum Answer {
    None,
    Ok,
    Cancel,
}

#[derive(Debug, Clone)]
pub struct PopUpState {
    pub show_popup: bool,
    pub message: String,
    pub message_type: MessageType,
    pub answer: Answer,
}

impl PopUpState {
    pub fn init() -> PopUpState {
        PopUpState {
            show_popup: false,
            message: String::from(""),
            message_type: MessageType::None,
            answer: Answer::None,
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageType::None => write!(f, ""),
            MessageType::Error => write!(f, " Error "),
            MessageType::Confirmation => write!(f, " Warning "),
        }
    }
}
