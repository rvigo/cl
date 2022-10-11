use anyhow::{ensure, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialOrd, Ord)]
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
        self.namespace = namespace.trim().to_string();
        self
    }
    pub fn alias(&mut self, alias: String) -> &mut CommandBuilder {
        self.alias = alias.trim().to_string();
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
            .sorted()
            .join(", ")
    }

    pub fn is_empty(&self) -> bool {
        self.namespace.is_empty() || self.alias.is_empty() || self.command.is_empty()
    }

    pub fn validate(&self) -> Result<()> {
        ensure!(
            !self.is_empty(),
            "namespace, command and alias field cannot be empty!"
        );
        ensure!(
            !self.alias.to_lowercase().contains(' '),
         "the alias must not contain whitespace as the application may interpret some words as arguments");

        Ok(())
    }
}

impl Default for Command {
    fn default() -> Self {
        Command {
            namespace: String::from(""),
            command: String::from("your command string goes here"),
            description: Some(String::from(
                "This is a demo entry and will be removed as soon you save your first command.
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

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.alias.eq(&other.alias) && self.namespace.eq(&other.namespace)
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Alias: {}\nNamespace: {}\nDescription: {}\nTags: {}\nCommand: {}",
            self.alias,
            self.namespace,
            self.description.as_ref().unwrap_or(&String::default()),
            self.tags_as_string(),
            self.command,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn build_default_command() -> Command {
        let mut command = CommandBuilder::default();
        command
            .tags(Some(vec![String::from("tag1")]))
            .alias(String::from("alias"))
            .namespace(String::from("namespace"))
            .description(Some(String::from("description")))
            .command(String::from("command"));

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
    fn should_not_validate_the_command_with_invalid_alias() {
        let mut invalid_command = CommandBuilder::default();
        invalid_command
            .tags(Some(vec![String::from("tag1")]))
            .alias(String::from("invalid alias"))
            .namespace(String::from("namespace"))
            .description(Some(String::from("description")))
            .command(String::from("command"));

        let invalid_command = invalid_command.build();

        let result = invalid_command.validate();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert_eq!(
            "the alias must not contain whitespace as the application may interpret some words as arguments",
            error_msg
        )
    }

    #[test]
    fn should_not_validate_the_command_with_missing_mandatory_field() {
        let mut invalid_command = CommandBuilder::default();
        invalid_command
            .tags(Some(vec![String::from("tag1")]))
            .alias(String::from("alias"))
            .description(Some(String::from("description")))
            .command(String::from("command"));

        let invalid_command = invalid_command.build();

        let result = invalid_command.validate();
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert_eq!(
            "namespace, command and alias field cannot be empty!",
            error_msg
        )
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
