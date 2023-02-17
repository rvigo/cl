use crate::{command::Command, commands::Commands, resources};
use anyhow::{anyhow, bail, Result};
use clap::Parser;
use log::debug;
use regex::Regex;
use std::collections::HashMap;
use strfmt::strfmt;

const DEFAULT_NAMED_PARAMS_ERROR_MESSAGE: &str = "This command has named parameters! \
You should provide them exactly as in the command";
const INVALID_NAMED_PARAMS_ERROR_MESSAGE: &str = "Invalid named arguments! \
    You should provide them exactly as in the command";

#[derive(Parser)]
pub struct Exec {
    #[clap(required = true, help = "The alias to be executed")]
    alias: String,
    #[clap(
        short,
        long,
        requires = "alias",
        required = false,
        help = "The namespace in case of duplicated aliases"
    )]
    namespace: Option<String>,
    #[clap(
        short,
        long,
        action,
        help = "Dry run mode (Just prints the alias command in the terminal)"
    )]
    dry_run: bool,
    #[clap(
        short,
        long,
        action,
        help = "Quiet mode (Prints only the command execution)"
    )]
    quiet: bool,

    #[clap(
        num_args(1..),
        last = true,
        requires = "alias",
        value_name = "COMMAND PARAMETERS",
        help = "The command args and/or named parameters.\n   \
            e.g: cl exec <some-alias> -- --named-parameter value --program_flag --program_option=yes"
    )]
    command_args: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct CommandArg {
    arg: String,
    prefix: String,
    value: Option<String>,
    named_parameter: bool,
}

impl CommandArg {
    fn as_key_value_pair(&self) -> (String, String) {
        let key = self.arg.to_string();
        let value = if let Some(value) = &self.value {
            value.to_string()
        } else {
            String::default()
        };
        (key, value)
    }
}

impl ToString for CommandArg {
    fn to_string(&self) -> String {
        if let Some(value) = &self.value {
            format!("{}{}={}", self.prefix, self.arg, value)
        } else {
            format!("{}{}", self.prefix, self.arg)
        }
    }
}

struct CommandArgs {
    command_args: Vec<CommandArg>,
}

impl CommandArgs {
    pub fn new(command_args: Vec<CommandArg>) -> Self {
        Self { command_args }
    }

    fn mark_named_parameters(&mut self, named_parameters: &[String]) {
        for command_arg in &mut self.command_args {
            if named_parameters.contains(&command_arg.arg) {
                command_arg.named_parameter = true;
            }
        }
    }

    fn get_named_parameters(&self) -> Vec<CommandArg> {
        self.command_args
            .clone()
            .into_iter()
            .filter(|a: &CommandArg| a.named_parameter)
            .collect()
    }

    fn get_options_as_string(&self) -> String {
        self.command_args
            .clone()
            .into_iter()
            .filter(|a: &CommandArg| !a.named_parameter)
            .map(|a: CommandArg| a.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl From<Vec<String>> for CommandArgs {
    fn from(args: Vec<String>) -> Self {
        let mut mapped_args = vec![];

        for arg in args {
            let mut command_arg = CommandArg::default();
            let parts: Vec<&str> = arg.splitn(2, '=').collect();
            if parts[0].starts_with("--") {
                command_arg.arg = parts[0].replacen("--", "", 1);
                command_arg.prefix = "--".to_string();
            } else {
                command_arg.arg = parts[0].to_string();
            }

            command_arg.value = if parts.len() > 1 {
                Some(parts[1].to_string())
            } else {
                None
            };
            mapped_args.push(command_arg);
        }

        CommandArgs::new(mapped_args)
    }
}

pub fn exec_subcommand(exec: Exec) -> Result<()> {
    let commands = Commands::init(resources::load_commands()?);

    let alias: String = exec.alias;
    let namespace: Option<String> = exec.namespace;
    let args: Vec<String> = exec.command_args;
    let dry_run: bool = exec.dry_run;
    let quiet_mode: bool = exec.quiet;
    let mut command_item: Command = commands.find_command(alias, namespace)?;

    command_item.command = prepare_command(command_item.command, args)?;
    debug!("command to be executed: {}", command_item.command);
    commands.exec_command(&command_item, dry_run, quiet_mode)
}

fn prepare_command(mut command: String, args: Vec<String>) -> Result<String> {
    // check if cmd has named_parameters
    let re = Regex::new(r"#\{[^\}]+\}").map_err(|e| anyhow!(e))?;
    let command_ref: &str = command.as_str();
    let matches: Vec<&str> = re.find_iter(command_ref).map(|m| m.as_str()).collect();
    let mut commands_args: CommandArgs = CommandArgs::from(args);

    //cmd does have named_parameter
    if !matches.is_empty() {
        //extract named_parameter
        let cleaned_matches: Vec<String> = matches
            .into_iter()
            .map(|m: &str| clean_named_parameter(m.to_string()))
            .collect();
        commands_args.mark_named_parameters(&cleaned_matches);
        let named_parameters = commands_args.get_named_parameters();
        let named_parameters: HashMap<String, String> = named_parameters
            .into_iter()
            .filter_map(|a: CommandArg| {
                if a.named_parameter {
                    Some(a.as_key_value_pair())
                } else {
                    None
                }
            })
            .collect();

        validate_named_parameters(&named_parameters, &command)?;

        // replace placeholders
        command = command.replace('#', "");
        command = strfmt(&command, &named_parameters)?
    }

    // options/args?
    let options = commands_args.get_options_as_string();
    if !options.is_empty() {
        let command = format!("{command} {options}");
        Ok(command)
    } else {
        Ok(command)
    }
}

fn clean_named_parameter(arg: String) -> String {
    arg.trim_matches(|c| c == '#' || c == '{' || c == '}')
        .to_string()
}

fn validate_named_parameters(mapped_args: &HashMap<String, String>, command: &str) -> Result<()> {
    let mut error_message: &str = "";
    if mapped_args.is_empty() {
        error_message = DEFAULT_NAMED_PARAMS_ERROR_MESSAGE;
    } else if mapped_args.iter().any(|(k, _)| k.is_empty()) {
        error_message = INVALID_NAMED_PARAMS_ERROR_MESSAGE;
    }
    if !error_message.is_empty() {
        bail!(format!(
            "Cannot run the command `{command}`\n\n{error_message}"
        ))
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_prepare_a_simple_command() {
        let result = prepare_command(String::from("echo hello"), vec![]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "echo hello");
    }

    #[test]
    fn should_prepare_a_command_with_named_parameters() {
        let result = prepare_command(
            String::from("echo #{name}"),
            vec![String::from("--name=unit_test")],
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "echo unit_test");
    }

    #[test]
    fn should_return_error_when_an_invalid_named_parameter_is_given() {
        let named_parameters: Vec<String> = vec![String::from("--invalid=unit_test")];
        let command = String::from("echo #{name}");
        let result: Result<String> = prepare_command(command.clone(), named_parameters);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("Cannot run the command `{command}`\n\n{DEFAULT_NAMED_PARAMS_ERROR_MESSAGE}"),
        );
    }
}
