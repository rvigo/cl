use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
pub enum Choice {
    Ok,
    Cancel,
}

impl Choice {
    pub fn dialog() -> Vec<Choice> {
        vec![Choice::Ok, Choice::Cancel]
    }

    pub fn confirm() -> Vec<Choice> {
        vec![Choice::Ok]
    }

    pub fn empty() -> Vec<Choice> {
        vec![]
    }
}

impl Display for Choice {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Choice::Ok => write!(f, "Ok"),
            Choice::Cancel => write!(f, "Cancel"),
        }
    }
}
