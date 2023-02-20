use crate::fuzzy::Fuzzy;
use anyhow::{ensure, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialOrd, Ord)]
pub struct Command {
    pub namespace: String,
    pub command: String,
    pub description: Option<String>,
    pub alias: String,
    pub tags: Option<Vec<String>>,
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

    pub fn description(&self) -> String {
        self.description
            .as_ref()
            .unwrap_or(&String::from(""))
            .to_string()
    }

    pub fn is_incomplete(&self) -> bool {
        self.namespace.trim().is_empty()
            || self.alias.trim().is_empty()
            || self.command.trim().is_empty()
    }

    pub fn validate(&self) -> Result<()> {
        ensure!(
            !self.is_incomplete(),
            "namespace, command and alias field cannot be empty!"
        );
        ensure!(
            !self.alias.contains(' '),
         "the alias must not contain whitespace as the application may interpret some words as arguments");
        Ok(())
    }

    pub fn has_named_parameter(&self) -> bool {
        self.command.contains("#{")
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

impl Fuzzy for Command {
    fn lookup_string(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.alias,
            self.namespace,
            self.description.as_ref().unwrap_or(&String::default()),
            self.tags_as_string(),
            self.command,
        )
    }
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
    pub fn namespace<T>(&mut self, namespace: T) -> &mut CommandBuilder
    where
        T: Into<String>,
    {
        self.namespace = namespace.into().trim().to_string();
        self
    }

    pub fn alias<T>(&mut self, alias: T) -> &mut CommandBuilder
    where
        T: Into<String>,
    {
        self.alias = alias.into().trim().to_string();
        self
    }

    pub fn command<T>(&mut self, command: T) -> &mut CommandBuilder
    where
        T: Into<String>,
    {
        self.command = command.into();
        self
    }

    pub fn description<T>(&mut self, description: Option<T>) -> &mut CommandBuilder
    where
        T: Into<String>,
    {
        self.description = description.map(|d| d.into());
        self
    }

    pub fn tags<T, S, I>(&mut self, tags: Option<T>) -> &mut CommandBuilder
    where
        T: IntoIterator<Item = S, IntoIter = I>,
        S: Into<String>,
        I: Iterator<Item = S>,
    {
        self.tags = tags.map(|v| v.into_iter().map(|t| t.into()).collect());
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

#[cfg(test)]
mod test {
    use super::*;

    fn build_default_command() -> Command {
        let mut command = CommandBuilder::default();
        command
            .tags(Some(vec!["tag1"]))
            .alias("alias")
            .namespace("namespace")
            .description(Some("description"))
            .command("command");

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
            .tags(Some(vec!["tag1"]))
            .alias("invalid lias")
            .namespace("namespace")
            .description(Some("description"))
            .command("command");

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
            .tags(Some(vec!["tag1"]))
            .alias("alias")
            .description(Some("description"))
            .command("command");

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
