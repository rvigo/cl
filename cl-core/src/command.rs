use crate::resources::errors::CommandError;
use anyhow::{ensure, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialOrd, Ord)]
pub struct Command {
    /// The command's alias. Is a `required` field and should not have empty spaces in it
    pub alias: String,
    /// The command's namespace. Is a `required` field and should not have empty spaces in it
    pub namespace: String,
    /// The command itself. Is a `required` field and can have multiple lines
    pub command: String,
    /// The command's description. Not a required field
    pub description: Option<String>,
    /// The command's tags. Not a required field
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
        ensure!(!self.is_incomplete(), CommandError::EmptyCommand);
        ensure!(
            !self.alias.trim().contains(' '),
            CommandError::AliasWithWhitespaces
        );
        ensure!(
            !self.namespace.trim().contains(' '),
            CommandError::NamespaceWithWhitespaces
        );
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
            command: String::from("echo \"this is your command\""),
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
        self.namespace = namespace.into().trim().to_owned();
        self
    }

    pub fn alias<T>(&mut self, alias: T) -> &mut CommandBuilder
    where
        T: Into<String>,
    {
        self.alias = alias.into().trim().to_owned();
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
            .description(Some("multiline\ndescription"))
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
    fn should_get_namespace_as_string() {
        let command = build_default_command();
        let description = command.description();
        assert_eq!(description, "multiline\ndescription")
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
        assert_eq!(CommandError::AliasWithWhitespaces.to_string(), error_msg)
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
        assert_eq!(CommandError::EmptyCommand.to_string(), error_msg)
    }

    #[test]
    fn should_return_an_error_when_command_is_not_valid() {
        let mut command = build_default_command();
        command.alias = String::from("");
        command.command = String::from("");
        command.namespace = String::from("");

        assert!(command.validate().is_err());
    }

    #[test]
    fn should_return_a_boolean_based_on_named_parameters() {
        let mut command = build_default_command();

        assert!(!command.has_named_parameter());

        command.command = String::from("echo \"hello, #{name}\"");

        assert!(command.has_named_parameter())
    }
}
