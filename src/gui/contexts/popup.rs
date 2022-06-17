use std::fmt;

use tui::widgets::ListState;

#[derive(Debug, Clone)]
pub enum MessageType {
    None,
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub enum Answer {
    None,
    Ok,
    Cancel,
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Answer::None => write!(f, ""),
            Answer::Ok => write!(f, "Ok"),
            Answer::Cancel => write!(f, "Cancel"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PopUp {
    pub popup: bool,
    pub message: String,
    pub message_type: MessageType,
    pub answer: Answer,
    pub options: Vec<Answer>,
    pub options_state: ListState,
}

impl PopUp {
    pub fn init() -> PopUp {
        PopUp {
            popup: false,
            message: String::from(""),
            message_type: MessageType::None,
            answer: Answer::None,
            options: vec![],
            options_state: ListState::default(),
        }
    }

    pub fn clear(&mut self) {
        self.message.clear();
        self.popup = false;
        self.message_type = MessageType::None;
        self.answer = Answer::None;
        self.options.clear();
        self.options_state.select(Some(0));
    }

    pub fn next(&mut self) {
        let i = match self.options_state.selected() {
            Some(i) => {
                if i >= self.options.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.options_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.options_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.options.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.options_state.select(Some(i));
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageType::None => write!(f, ""),
            MessageType::Error => write!(f, " Error "),
            MessageType::Warning => write!(f, " Warning "),
        }
    }
}
