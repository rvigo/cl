mod maybe_stdin;

use super::Subcommand;
use anyhow::Result;
use cl_core::{fs, initialize_commands, CommandBuilder, Config};
use clap::Parser;
use log::{info, warn};
use maybe_stdin::MaybeStdin;

#[derive(Parser, Debug)]
pub struct Add {
    #[clap(
        default_value = "",
        help = "The command to be added (may be read from stdin)"
    )]
    command: MaybeStdin<String>,
}

impl Subcommand for Add {
    fn run(&self, config: impl Config) -> Result<()> {
        let command_string = self.command.value.to_owned();

        if command_string.is_empty() {
            warn!("No command provided. Use `cl add --help` for more information.");
            return Ok(());
        }

        let alias: String = command_string.chars().take(5).collect();

        const DEFAULT_NAMESPACE: &str = "from_stdin";

        let builder = CommandBuilder::default();
        let command = builder
            .command(command_string.to_owned())
            .alias(alias.to_owned())
            .namespace(DEFAULT_NAMESPACE.to_owned())
            .build();

        let mut commands = initialize_commands!(config.command_file_path());

        let result = commands.add(&command)?;
        fs::save_at(result, config.command_file_path())?;

        if !config.preferences().quiet_mode() {
            info!("Command added: {}", command_string);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    /// Mirrors the alias-generation logic in `run()` so it can be tested in isolation.
    fn generate_alias(command: &str) -> String {
        command.chars().take(5).collect()
    }

    #[test]
    fn should_take_first_5_chars_for_alias() {
        assert_eq!(generate_alias("echo hello world"), "echo ");
    }

    #[test]
    fn should_use_full_string_when_shorter_than_5_chars() {
        assert_eq!(generate_alias("rm"), "rm");
    }

    #[test]
    fn should_handle_multibyte_chars_without_panicking() {
        // Each 'é' is 2 bytes; byte-level truncate(5) would panic mid-char
        let alias = generate_alias("écho hello");
        assert_eq!(alias, "écho ");
        assert!(alias.is_char_boundary(alias.len()));
    }
}
