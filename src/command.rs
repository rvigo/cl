use anyhow::{bail, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Command {
    pub namespace: String,
    pub command: String,
    pub description: Option<String>,
    pub alias: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Default)]
pub struct CommandBuilder {
    namespace: String,
    command: String,
    description: Option<String>,
    alias: String,
    tags: Option<Vec<String>>,
}

impl CommandBuilder {
    pub fn namespace(&mut self, namespace: String) -> &mut CommandBuilder {
        self.namespace = namespace;
        self
    }
    pub fn alias(&mut self, alias: String) -> &mut CommandBuilder {
        self.alias = alias;
        self
    }
    pub fn command(&mut self, command: String) -> &mut CommandBuilder {
        self.command = command;
        self
    }
    pub fn description(&mut self, description: Option<String>) -> &mut CommandBuilder {
        self.description = description;
        self
    }
    pub fn tags(&mut self, tags: Option<Vec<String>>) -> &mut CommandBuilder {
        self.tags = tags;
        self
    }

    pub fn build(self) -> Command {
        Command {
            namespace: self.namespace,
            command: self.command,
            description: self.description,
            alias: self.alias,
            tags: self.tags,
        }
    }
}

impl Command {
    pub fn tags_as_string(&self) -> String {
        self.tags
            .as_ref()
            .unwrap_or(&vec![String::from("")])
            .iter()
            .join(", ")
    }

    pub fn is_empty(&self) -> bool {
        self.namespace.is_empty() && self.alias.is_empty() && self.command.is_empty()
    }

    pub fn validate(&self) -> Result<()> {
        if self.is_empty() {
            bail!("namespace, command and alias field cannot be empty!");
        }

        Ok(())
    }
}

impl Default for Command {
    fn default() -> Self {
        Command {
            namespace: String::from(""),
            command: String::from("your command string goes here"),
            description: Some(String::from(
                "This is a demo entry and will be deleted as soon you save your first command.
                Also, a nice description of your command goes here (optional)",
            )),
            alias: String::from("your command alias"),
            tags: Some(vec![
                String::from("optional"),
                String::from("tags"),
                String::from("comma"),
                String::from("separated"),
            ]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn build_default_command() -> Command {
        let mut command = CommandBuilder::default();
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
        let command = build_default_command();
        let tags = command.tags_as_string();
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
