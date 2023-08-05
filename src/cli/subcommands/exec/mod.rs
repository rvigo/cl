pub mod command_args;

use self::command_args::CommandArg;
use super::Subcommand;
use crate::{
    load_commands,
    resources::{config::Config, errors::CommandError, logger::interceptor::ErrorInterceptor},
};
use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use command_args::CommandArgs;
use itertools::Itertools;
use log::debug;
use regex::Regex;
use std::collections::HashMap;
use strfmt::strfmt;
use thiserror::Error;

const DEFAULT_NAMED_PARAMS_ERROR_MESSAGE: &str = "This command has named parameters! \
You should provide them exactly as in the command";
const INVALID_NAMED_PARAMS_ERROR_MESSAGE: &str = "Invalid named arguments! \
    You should provide them exactly as in the command";

#[derive(Error, Debug)]
enum ExecError {
    #[error("Missing a named parameter: {missing_parameter}")]
    MissingNamedParameter { missing_parameter: String },
    #[error("An error ocurred while parsing your command")]
    GenericError,
}

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

impl Subcommand for Exec {
    fn run(&self, config: Config) -> Result<()> {
        let commands = load_commands!(config.get_command_file_path()).log_error()?;
        let alias = &self.alias;
        let namespace = &self.namespace;
        let args = &self.command_args;
        let dry_run = self.dry_run;
        let quiet_mode = self.quiet || config.get_quiet_mode();
        let mut command_item = commands.find_command(alias.to_owned(), namespace.to_owned())?;

        command_item.command = prepare_command(command_item.command, args.to_owned())
            .context("Cannot prepare the command to be executed")
            .log_error()?;

        debug!("command to be executed: {}", command_item.command);
        commands
            .exec_command(&command_item, dry_run, quiet_mode)
            .context("Cannot run the command")
            .log_error()
    }
}

// TODO it should use a `&str`
fn prepare_command(mut command: String, args: Vec<String>) -> Result<CommandString> {
    // checks if cmd has named_parameters
    let matches = get_named_parameters(&command)?;
    let named_parameters = matches
        .iter()
        .map(|m| clean_named_parameter(m))
        .collect_vec();

    let mut command_args = CommandArgs::init(named_parameters);

    // cmd does have named_parameter
    if command_args.has_named_parameters() {
        let mut peekable_args = args.into_iter().peekable();
        let mut last_arg = String::default();

        for arg in peekable_args.clone() {
            // if the current item is the same as last item processed, skips
            if arg == last_arg {
                continue;
            }

            let command_arg = if !arg.contains('=') {
                if last_arg.is_empty() {
                    peekable_args.next();
                }

                if let Some(next) = peekable_args.peek() {
                    if !next.starts_with("--") && arg.starts_with("--") {
                        let arg = arg.replacen("--", "", 1);
                        let prefix = Some("--".to_owned());

                        // peeks the next item
                        let next_item = peekable_args.next();
                        let value = next_item.clone();

                        // and set it as the `last item` processed
                        if let Some(next_item) = next_item {
                            last_arg = next_item;
                        }

                        peekable_args.next();

                        CommandArg::new(arg, prefix, value)
                    } else if next.starts_with("--") && !arg.starts_with("--") {
                        let arg = arg.replacen("--", "", 1);
                        let prefix = Some("--".to_owned());
                        let value = None;

                        peekable_args.next();

                        CommandArg::new(arg, prefix, value)
                    } else if next.starts_with("--") && arg.starts_with("--") {
                        // it this a flag???
                        let arg = arg.replacen("--", "", 1);
                        let prefix = Some("--".to_owned());
                        let value = None;

                        CommandArg::new(arg, prefix, value)
                    } else if !next.starts_with("--") && !arg.starts_with("--") {
                        // is this a subcommand???
                        let prefix = None;
                        let value = None;

                        CommandArg::new(arg, prefix, value)
                    } else {
                        // wtf is this???
                        CommandArg::default()
                    }
                } else {
                    let (arg, prefix) = if arg.starts_with("--") {
                        (arg.replacen("--", "", 1), Some("--".to_owned()))
                    } else {
                        (arg, None)
                    };
                    peekable_args.next();
                    CommandArg::new(arg, prefix, None)
                }
            } else {
                let parts: Vec<&str> = arg.splitn(2, '=').collect();
                let (arg, prefix) = if parts[0].starts_with("--") {
                    (parts[0].replacen("--", "", 1), Some("--".to_owned()))
                } else {
                    (parts[0].to_owned(), None)
                };

                let value = if parts.len() > 1 {
                    Some(parts[1].to_owned())
                } else {
                    None
                };

                CommandArg::new(arg, prefix, value)
            };

            if !command_arg.is_empty() {
                command_args.push(command_arg);
            }
        }

        if let Some(parameters) = command_args.named_parameters() {
            let named_parameters = parameters
                .iter()
                .map(|a| a.as_key_value_pair())
                .collect::<HashMap<String, String>>();

            validate_named_parameters(&named_parameters, &command)
                .context("Cannot validate the named parameters")?;

            command = replace_placeholders(command, &named_parameters)
                .context("Cannot replace the placeholders with the provided args")?;
        } else {
            bail!(CommandError::CannotRunCommand {
                command,
                cause: DEFAULT_NAMED_PARAMS_ERROR_MESSAGE.to_owned()
            })
        }
        // options/args/flags
        let options = command_args
            .options()
            .unwrap_or(&vec![])
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>();

        command = append_options(&command, options);

        Ok(command)
    } else {
        let command = append_options(&command, args);

        Ok(command)
    }
}

fn get_named_parameters(command: &str) -> Result<Vec<String>> {
    let re = Regex::new(r"#\{[^\}]+\}").map_err(|e| anyhow!(e))?;
    let matches = re
        .find_iter(command)
        .map(|m| String::from(m.as_str()))
        .collect_vec();

    Ok(matches)
}

fn append_options(command: &str, options: Vec<String>) -> CommandString {
    if !options.is_empty() {
        let command = format!("{command} {}", options.join(" "));
        command
    } else {
        command.to_owned()
    }
}

fn replace_placeholders(
    mut command: CommandString,
    named_parameters: &HashMap<String, String>,
) -> Result<CommandString> {
    command = command.replace('#', "");
    let parse_result = match strfmt(&command, named_parameters) {
        Ok(c) => c,
        Err(error) => {
            let res = match error {
                strfmt::FmtError::KeyError(message) => {
                    let missing_key = message.split(':').collect_vec()[1].trim();
                    ExecError::MissingNamedParameter {
                        missing_parameter: missing_key.to_owned(),
                    }
                }
                _ => ExecError::GenericError,
            };
            bail!(res)
        }
    };
    Ok(parse_result)
}

fn clean_named_parameter(arg: &str) -> CommandString {
    arg.trim_matches(|c| c == '#' || c == '{' || c == '}')
        .to_owned()
}

fn validate_named_parameters(mapped_args: &HashMap<String, String>, command: &str) -> Result<()> {
    let mut error_message: &str = "";

    if mapped_args.is_empty() {
        error_message = DEFAULT_NAMED_PARAMS_ERROR_MESSAGE;
    } else if mapped_args.iter().any(|(k, _)| k.is_empty()) {
        error_message = INVALID_NAMED_PARAMS_ERROR_MESSAGE;
    }

    if !error_message.is_empty() {
        bail!(CommandError::CannotRunCommand {
            command: command.to_owned(),
            cause: error_message.to_owned()
        })
    }

    Ok(())
}

type CommandString = String;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_prepare_a_simple_command() {
        let result = prepare_command(String::from("echo hello"), vec![]);
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
            CommandError::CannotRunCommand {
                command,
                cause: DEFAULT_NAMED_PARAMS_ERROR_MESSAGE.to_owned()
            }
            .to_string()
        );
    }

    #[test]
    fn should_append_the_options_to_the_command() {
        let command = String::from("echo Hello");
        let args = vec![String::from("World")];
        let result = prepare_command(command.clone(), args.clone());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), format!("{} {}", command, args[0]));
    }
}
