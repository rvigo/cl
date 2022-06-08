use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct CommandItem {
    pub namespace: String,
    pub command: String,
    pub description: Option<String>,
    pub alias: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl CommandItem {
    pub fn tags_str(&mut self) -> String {
        self.tags
            .as_ref()
            .unwrap_or(&vec![String::from("")])
            .into_iter()
            .join(", ")
    }
}
