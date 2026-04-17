mod maybe_stdin;

use super::Subcommand;
use anyhow::Result;
use cl_core::{fs, initialize_commands, CommandBuilder, Config};
use clap::Parser;
use maybe_stdin::MaybeStdin;
use tracing::{info, warn};

fn generate_alias(command: &str) -> String {
    command
        .split_whitespace()
        .next()
        .unwrap_or("")
        .chars()
        .take(5)
        .collect()
}

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
            warn!(target: "cl::add", "no command provided; run `cl add --help` for usage");
            return Ok(());
        }

        let alias = generate_alias(&command_string);

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
        info!(target: "cl::add", alias = %alias, command = %command_string, "command added");

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::generate_alias;

    #[test]
    fn should_take_first_5_chars_of_first_word_for_alias() {
        assert_eq!(generate_alias("echo hello world"), "echo");
    }

    #[test]
    fn should_take_first_5_chars_when_first_word_is_long() {
        assert_eq!(generate_alias("echoo hello"), "echoo");
    }

    #[test]
    fn should_use_full_first_word_when_shorter_than_5_chars() {
        assert_eq!(generate_alias("rm -rf /tmp"), "rm");
    }

    #[test]
    fn should_return_empty_string_for_empty_input() {
        assert_eq!(generate_alias(""), "");
    }

    #[test]
    fn should_return_empty_string_for_whitespace_only_input() {
        assert_eq!(generate_alias("   "), "");
    }

    #[test]
    fn should_handle_multibyte_chars_without_panicking() {
        // Each 'é' is 2 bytes; byte-level truncate(5) would panic mid-char
        let alias = generate_alias("écho hello");
        assert_eq!(alias, "écho");
        assert!(alias.is_char_boundary(alias.len()));
    }
}
