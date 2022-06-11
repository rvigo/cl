use anyhow::{anyhow, Result};
use itertools::Itertools;
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct CommandItem {
    pub namespace: String,
    pub command: String,
    pub description: Option<String>,
    pub alias: String,
    pub tags: Option<Vec<String>>,
}

pub struct CommandItemBuilder {
    namespace: String,
    command: String,
    description: Option<String>,
    alias: String,
    tags: Option<Vec<String>>,
}

impl Default for CommandItemBuilder {
    fn default() -> Self {
        Self {
            namespace: Default::default(),
            command: Default::default(),
            description: Default::default(),
            alias: Default::default(),
            tags: Default::default(),
        }
    }
}
impl CommandItemBuilder {
    pub fn namespace(&mut self, namespace: String) -> &mut CommandItemBuilder {
        self.namespace = namespace;
        self
    }
    pub fn alias(&mut self, alias: String) -> &mut CommandItemBuilder {
        self.alias = alias;
        self
    }
    pub fn command(&mut self, command: String) -> &mut CommandItemBuilder {
        self.command = command;
        self
    }
    pub fn description(&mut self, description: Option<String>) -> &mut CommandItemBuilder {
        self.description = description;
        self
    }
    pub fn tags(&mut self, tags: Option<Vec<String>>) -> &mut CommandItemBuilder {
        self.tags = tags;
        self
    }

    pub fn build(self) -> CommandItem {
        CommandItem {
            namespace: self.namespace,
            command: self.command,
            description: self.description,
            alias: self.alias,
            tags: self.tags,
        }
    }
}

impl CommandItem {
    pub fn tags_str(&mut self) -> String {
        self.tags
            .as_ref()
            .unwrap_or(&vec![String::from("")])
            .into_iter()
            .join(", ")
    }

    pub fn validate(&self) -> Result<()> {
        info!("self: {:#?}", self);
        if self.namespace.is_empty() || self.command.is_empty() || self.alias.is_empty() {
            error!("namespace, command and alias field cannot be empty!");
            return Err(anyhow!(
                "namespace, command and alias field cannot be empty!"
            ));
        }

        Ok(())
    }
}
