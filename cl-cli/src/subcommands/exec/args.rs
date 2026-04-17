use anyhow::{bail, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::sync::LazyLock;

static NAMED_PARAM_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"#\{[^}]+\}").expect("Invalid regex pattern"));

const ARG_PREFIX: &str = "--";

#[derive(Debug, Default)]
pub struct CommandArgs {
    named: Vec<CommandArg>,
    non_named: Vec<CommandArg>,
    /// Reference list with collected named parameters keys
    named_parameters: HashSet<String>,
}

impl CommandArgs {
    pub fn init(command: &str, args: Vec<String>) -> Result<CommandArgs> {
        let named_parameters = Self::filter_named_parameters(command)?;

        let mut command_args = Self {
            named: Vec::new(),
            non_named: Vec::new(),
            named_parameters,
        };

        for arg in args {
            let mut parts = arg.splitn(2, '=');
            let key = parts.next().unwrap_or("");
            let value = parts.next().map(|v| v.to_string());

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
        }

        let named_len = command_args.named.len();
        let named_parameters_len = command_args.named_parameters.len();

        if named_len != named_parameters_len {
            let provided: HashSet<String> =
                command_args.named.iter().map(|a| a.arg.clone()).collect();
            let mut missing: Vec<&String> = command_args
                .named_parameters
                .difference(&provided)
                .collect();
            missing.sort();
            bail!(
                "Missing named parameters: {}",
                missing
                    .iter()
                    .map(|s| format!("#{{{s}}}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        Ok(command_args)
    }

    pub(super) fn named_parameters_map(&self) -> Option<HashMap<String, String>> {
        if self.named.is_empty() {
            return None;
        }
        Some(
            self.named
                .iter()
                .map(|a| a.as_key_value_pair())
                .collect::<HashMap<String, String>>(),
        )
    }

    pub(super) fn options(&self) -> Option<&Vec<CommandArg>> {
        if self.non_named.is_empty() {
            None
        } else {
            Some(&self.non_named)
        }
    }

    fn push(&mut self, command_arg: CommandArg) {
        if self.named_parameters.contains(&command_arg.arg) {
            self.named.push(command_arg);
        } else {
            self.non_named.push(command_arg);
        }
    }

    fn filter_named_parameters(command: &str) -> Result<HashSet<String>> {
        let matches = NAMED_PARAM_REGEX
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

impl CommandArg {
    pub fn new(arg: String, prefix: Option<String>, value: Option<String>) -> CommandArg {
        Self { arg, prefix, value }
    }

    pub fn as_key_value_pair(&self) -> (String, String) {
        let key = self.arg.clone();
        let value = self.value.clone().unwrap_or_default();
        (key, value)
    }

    pub fn is_empty(&self) -> bool {
        self.arg.is_empty() && self.prefix.as_deref().is_none_or(|p| p.is_empty())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_preserve_value_containing_equals_sign() {
        let command = "curl #{url}";
        let args = vec!["--url=https://example.com?foo=bar&baz=qux".to_string()];
        let result = CommandArgs::init(command, args);
        assert!(result.is_ok());
        let map = result.unwrap().named_parameters_map().unwrap();
        assert_eq!(
            map.get("url").unwrap(),
            "https://example.com?foo=bar&baz=qux"
        );
    }

    #[test]
    fn should_report_missing_named_parameters_by_name() {
        let command = "echo #{greeting} #{name}";
        // only --greeting provided, --name is missing
        let args = vec!["--greeting=hello".to_string()];
        let result = CommandArgs::init(command, args);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("#{name}"),
            "Error should name the missing parameter, got: {err}"
        );
    }

    #[test]
    fn should_succeed_when_all_named_parameters_are_provided() {
        let command = "echo #{greeting} #{name}";
        let args = vec!["--greeting=hello".to_string(), "--name=world".to_string()];
        let result = CommandArgs::init(command, args);
        assert!(result.is_ok());
    }
}

impl Display for CommandArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = self.prefix.as_deref().unwrap_or_default();

        let str = match &self.value {
            Some(value) => format!("{}{}={}", prefix, self.arg, value),
            None => format!("{}{}", prefix, self.arg),
        };
        write!(f, "{str}")
    }
}
