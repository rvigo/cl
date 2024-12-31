use crate::CommandError;
use anyhow::{ensure, Result};
use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialOrd, Ord)]
pub struct Command<'cmd> {
    /// The command's alias. Is a `required` field and should not have empty spaces in it
    pub alias: Cow<'cmd, str>,
    /// The command's namespace. Is a `required` field and should not have empty spaces in it
    pub namespace: Cow<'cmd, str>,
    /// The command itself. Is a `required` field and can have multiple lines
    pub command: Cow<'cmd, str>,
    /// The command's description. Not a required field
    pub description: Option<Cow<'cmd, str>>,
    /// The command's tags. Not a required field
    pub tags: Option<Vec<Cow<'cmd, str>>>,
}

impl<'cmd> Command<'cmd> {
    pub fn tags_as_string(&self) -> String {
        self.tags
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .sorted()
            .join(", ")
    }

    pub fn description(&self) -> String {
        self.description
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default()
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
        let re = Regex::new(r"#\{[^}]*\}").expect("Invalid regex pattern");

        re.is_match(&self.command)
    }

    pub fn has_changes(&self, new: &Command) -> bool {
        new.alias != self.alias
            || new.command != self.command
            || new.description != self.description
            || new.tags != self.tags
            || new.namespace != self.namespace
    }

    fn is_incomplete(&self) -> bool {
        self.namespace.trim().is_empty()
            || self.alias.trim().is_empty()
            || self.command.trim().is_empty()
    }
}

impl Default for Command<'_> {
    fn default() -> Self {
        Command {
            namespace: Cow::Borrowed("Namespace"),
            command: Cow::Borrowed("echo \"this is your command\""),
            description: (Some(Cow::Borrowed(
                "This is a demo entry and will be removed as soon you save your first command.
                Also, a nice description of your command goes here (optional)",
            ))),
            alias: Cow::Borrowed("your command alias"),
            tags: Some(vec![
                Cow::Borrowed("optional"),
                Cow::Borrowed("tags"),
                Cow::Borrowed("comma"),
                Cow::Borrowed("separated"),
            ]),
        }
    }
}

impl PartialEq for Command<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.alias.eq(&other.alias) && self.namespace.eq(&other.namespace)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::CommandBuilder;

    fn build_default_command() -> Command<'static> {
        let command = CommandBuilder::default()
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
        let invalid_command = CommandBuilder::default()
            .tags(Some(vec!["tag1"]))
            .alias("invalid alias")
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
        let invalid_command = CommandBuilder::default()
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
        command.alias = Cow::Borrowed("");
        command.command = Cow::Borrowed("");
        command.namespace = Cow::Borrowed("");

        assert!(command.validate().is_err());
    }

    #[test]
    fn should_validate_if_command_has_named_parameters() {
        let mut command = build_default_command();

        assert!(!command.has_named_parameter());

        command.command = Cow::Borrowed("echo \"hello, #{name}\"");

        assert!(command.has_named_parameter())
    }
}
