use anyhow::{bail, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
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
        if self.namespace.is_empty() || self.command.is_empty() || self.alias.is_empty() {
            bail!("namespace, command and alias field cannot be empty!");
        }

        Ok(())
    }
}

impl Default for CommandItem {
    fn default() -> Self {
        CommandItem {
            namespace: String::from(""),
            command: String::from("your command string goes here"),
            description: Some(String::from(
                "a nice description of your command goes here (optional)",
            )),
            alias: String::from("your command alias"),
            tags: Some(vec![String::from("optional"), String::from("tags")]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn build_default_command() -> CommandItem {
        let mut command = CommandItemBuilder::default();
        command
            .tags(Some(vec![String::from("tag1")]))
            .alias(String::from("test alias"))
            .namespace(String::from("test namespace"))
            .description(Some(String::from("test description")))
            .command(String::from("test command"));

        command.build()
    }

    #[test]
    fn should_get_tags_as_str() {
        let mut command = build_default_command();
        let tags = command.tags_str();
        assert_eq!(String::from("tag1"), tags)
    }

    #[test]
    fn should_validate_the_command() {
        let command = build_default_command();

        assert!(command.validate().is_ok());
    }

    #[test]
    fn should_return_an_error_when_command_is_not_valid() {
        let mut command = build_default_command();
        command.alias = String::from("");
        command.command = String::from("");
        command.namespace = String::from("");

        assert!(command.validate().is_err());
    }
}
