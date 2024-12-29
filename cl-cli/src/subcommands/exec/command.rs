use super::args::CommandArgs;
use anyhow::{Context, Result};
use std::ops::Deref;
use strfmt::strfmt;

#[derive(Debug)]
pub struct Command {
    pub inner: String,
    pub args: CommandArgs,
}

impl Command {
    pub fn new(command: impl Into<String>, args: Vec<String>) -> Result<Self> {
        let command = command.into();
        let args = CommandArgs::init(&command, args).context("Cannot parse the given args")?;

        let cs = Self {
            inner: command,
            args,
        };

        let new_cs = cs
            .replace_placeholders()
            .context("Cannot replace the placeholders with the provided args")?
            .append_options();

        Ok(new_cs)
    }

    fn append_options(self) -> Self {
        if let Some(options) = self.args.options() {
            let filtered_options: Vec<String> = options
                .iter()
                .filter(|a| !a.is_empty())
                .map(|a| a.to_string())
                .collect();

            let command = format!("{} {}", self.inner.trim(), filtered_options.join(" "));

            Self {
                inner: command,
                args: self.args,
            }
        } else {
            self
        }
    }

    fn replace_placeholders(self) -> Result<Self> {
        let named_parameters_map = self.args.named_parameters_map();

        if let Some(options) = named_parameters_map {
            let new_command = self.inner.replace('#', "");
            let parse_result = strfmt(&new_command, &options)?;

            Ok(Self {
                inner: parse_result,
                args: self.args,
            })
        } else {
            Ok(self)
        }
    }
}

impl Deref for Command {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
