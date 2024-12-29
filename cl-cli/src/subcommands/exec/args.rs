use anyhow::{anyhow, bail, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};

const ARG_PREFIX: &str = "--";

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CommandArgType {
    Named,
    NonNamed,
}

#[derive(Debug, Default)]
pub struct CommandArgs {
    /// Hashmap with two groups (named parameters / non named parameters)
    args: HashMap<CommandArgType, Vec<CommandArg>>,
    /// Reference list with collected named parameters keys
    named_parameters: HashSet<String>,
}

impl CommandArgs {
    pub fn init(command: &str, args: Vec<String>) -> Result<CommandArgs> {
        let named_parameters = Self::filter_named_parameters(command)?;

        let mut command_args = Self {
            args: HashMap::default(),
            named_parameters,
        };

        args.into_iter().for_each(|arg| {
            let parts: Vec<&str> = arg.split('=').collect();
            let (key, value) = (parts[0], parts.get(1).map(|&v| v.to_string()));

            let (arg, prefix) = if key.starts_with(ARG_PREFIX) {
                (
                    key.trim_start_matches(ARG_PREFIX).to_string(),
                    Some(ARG_PREFIX.to_string()),
                )
            } else {
                (key.to_string(), None)
            };

            let command_arg = CommandArg::new(arg, prefix, value);
            command_args.push(command_arg);
        });

        let command_args_len = command_args
            .args
            .get(&CommandArgType::Named)
            .map_or(0, |c| c.len());

        let named_parameters_len = command_args.named_parameters.len();

        if command_args_len != named_parameters_len {
            bail!(
                "Some named parameters are missing: expected {}, found {}",
                named_parameters_len,
                command_args_len
            )
        };
        Ok(command_args)
    }

    pub(super) fn named_parameters_map(&self) -> Option<HashMap<String, String>> {
        self.args.get(&CommandArgType::Named).map(|c| {
            c.iter()
                .map(|a| a.as_key_value_pair())
                .collect::<HashMap<String, String>>()
        })
    }

    pub(super) fn options(&self) -> Option<&Vec<CommandArg>> {
        self.args.get(&CommandArgType::NonNamed)
    }

    fn push(&mut self, command_arg: CommandArg) {
        let key = if self.named_parameters.contains(&command_arg.arg) {
            CommandArgType::Named
        } else {
            CommandArgType::NonNamed
        };

        self.args.entry(key).or_default().push(command_arg);
    }

    fn filter_named_parameters(command: &str) -> Result<HashSet<String>> {
        let re = Regex::new(r"#\{[^\}]+\}").map_err(|e| anyhow!(e))?;
        let matches = re
            .find_iter(command)
            .map(|m| m.as_str().to_string())
            .collect::<HashSet<_>>();
        let named_parameters = matches.iter().map(Self::clean_named_parameter).collect();

        Ok(named_parameters)
    }

    fn clean_named_parameter(arg: impl Into<String>) -> String {
        arg.into()
            .trim_matches(|c| c == '#' || c == '{' || c == '}')
            .to_owned()
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CommandArg {
    arg: String,
    prefix: Option<String>,
    value: Option<String>,
}

pub type Pair<K, V> = (K, V);

impl CommandArg {
    pub fn new(arg: String, prefix: Option<String>, value: Option<String>) -> CommandArg {
        Self { arg, prefix, value }
    }

    pub fn as_key_value_pair(&self) -> Pair<String, String> {
        let key = self.arg.to_string();
        let value = self.value.to_owned().unwrap_or_default();
        (key, value)
    }

    pub fn is_empty(&self) -> bool {
        self.arg.is_empty() && self.prefix.as_deref().map_or(true, |p| p.is_empty())
    }
}

impl ToString for CommandArg {
    fn to_string(&self) -> String {
        let prefix = self.prefix.to_owned().unwrap_or_default();

        match &self.value {
            Some(value) => format!("{}{}={}", prefix, self.arg, value),
            None => format!("{}{}", prefix, self.arg),
        }
    }
}
