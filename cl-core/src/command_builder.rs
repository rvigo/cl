use crate::Command;
use std::borrow::Cow;

#[derive(Default)]
pub struct CommandBuilder {
    namespace: String,
    command: String,
    description: Option<String>,
    alias: String,
    tags: Option<Vec<String>>,
}

impl CommandBuilder {
    pub fn namespace<T>(mut self, namespace: T) -> CommandBuilder
    where
        T: Into<String>,
    {
        self.namespace = namespace.into().trim().to_owned();
        self
    }

    pub fn alias<T>(mut self, alias: T) -> CommandBuilder
    where
        T: Into<String>,
    {
        self.alias = alias.into().trim().to_owned();
        self
    }

    pub fn command<T>(mut self, command: T) -> CommandBuilder
    where
        T: Into<String>,
    {
        self.command = command.into();
        self
    }

    pub fn description<T>(mut self, description: Option<T>) -> CommandBuilder
    where
        T: Into<String>,
    {
        self.description = description.map(|d| d.into());
        self
    }

    pub fn tags<T, S, I>(mut self, tags: Option<T>) -> CommandBuilder
    where
        T: IntoIterator<Item = S, IntoIter = I>,
        S: Into<String>,
        I: Iterator<Item = S>,
    {
        self.tags = tags.map(|v| v.into_iter().map(|t| t.into()).collect());
        self
    }

    pub fn build<'a>(self) -> Command<'a> {
        Command {
            namespace: Cow::Owned(self.namespace),
            command: Cow::Owned(self.command),
            description: self.description.map(Cow::Owned),
            alias: Cow::Owned(self.alias),
            tags: self
                .tags
                .map(|vec| vec.into_iter().map(Cow::Owned).collect()),
        }
    }
}
