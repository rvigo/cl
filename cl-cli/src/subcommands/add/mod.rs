mod maybe_stdin;

use super::Subcommand;
use anyhow::Result;
use cl_core::{resource::FileService, CommandBuilder, Commands, Config};
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

        let mut alias = command_string.clone();
        alias.truncate(5);

        const DEFAULT_NAMESPACE: &str = "from_stdin";

        let builder = CommandBuilder::default();
        let command = builder
            .command(command_string.to_owned())
            .alias(alias.to_owned())
            .namespace(DEFAULT_NAMESPACE.to_owned())
            .build();

        let commands_service = FileService::new(config.command_file_path())?;
        let commands = commands_service.load()?;
        let mut commands = Commands::init(commands);

        let result = commands.add(&command)?;
        commands_service.save(&result)?;

        if !config.preferences().quiet_mode() {
            info!("Command added: {}", command_string);
        }

        Ok(())
    }
}
