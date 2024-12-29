pub mod args;
pub mod command;

use super::Subcommand;
use anyhow::{Context, Result};
use cl_core::{load_commands, CommandExec, Config};
use clap::Parser;
use command::Command;
use log::debug;

#[derive(Parser)]
pub struct Exec {
    #[clap(required = true, help = "The alias of the command to be executed")]
    alias: String,
    #[clap(
        short,
        long,
        requires = "alias",
        required = false,
        help = "The namespace to use in case of duplicate aliases"
    )]
    namespace: Option<String>,
    #[clap(
        short,
        long,
        action,
        help = "Dry run mode (prints the alias command in the terminal without executing it)"
    )]
    dry_run: bool,
    #[clap(
        short,
        long,
        action,
        help = "Quiet mode (prints only the command execution output)"
    )]
    quiet: bool,

    #[clap(
        num_args(1..),
        last = true,
        requires = "alias",
        value_name = "COMMAND PARAMETERS",
        help = "The command arguments and/or named parameters.\n   \
            e.g., cl exec <some-alias> -- --named-parameter value --program_flag --program_option=yes"
    )]
    command_args: Vec<String>,
}

impl Subcommand for Exec {
    fn run(&self, config: Config) -> Result<()> {
        let commands = load_commands!(config.command_file_path())?;
        let alias = &self.alias;
        let namespace = &self.namespace;
        let args = &self.command_args;
        let dry_run = self.dry_run;
        let quiet_mode = self.quiet || config.preferences().quiet_mode();

        let mut command_item = commands
            .find(alias, namespace.as_deref())
            .context("Failed to find the command with the given alias and namespace")?;

        let new_command = Command::new(command_item.command, args.clone())
            .context("Cannot prepare the command to be executed")?
            .inner;

        debug!("Command to be executed: {}", new_command);

        command_item.command = new_command;
        command_item
            .exec(dry_run, quiet_mode)
            .context("Cannot run the command")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_prepare_a_simple_command() {
        let command = "echo hello";
        let result = Command::new(command, vec![]);
        assert_eq!(*result.unwrap(), "echo hello");
    }

    #[test]
    fn should_prepare_a_command_with_named_parameters() {
        let command = "echo #{name}";
        let result = Command::new(
            command,
            vec![String::from("--name"), String::from("unit_test")],
        );
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), "echo unit_test");
    }

    #[test]
    fn should_return_error_when_an_invalid_named_parameter_is_given() {
        let named_parameters: Vec<String> = vec![String::from("--invalid=unit_test")];
        let command = "echo #{name}";
        let result = Command::new(command, named_parameters);

        assert!(result.is_err());

        assert_eq!(
            "Cannot parse the given args",
            result.unwrap_err().to_string()
        );
    }

    #[test]
    fn should_append_the_options_to_the_command() {
        let command = "echo Hello";
        let args = vec![String::from("World")];
        let result = Command::new(command, args.clone());

        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), format!("{} {}", command, args[0]));
    }
}
