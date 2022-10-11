use crate::resources::config;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[clap(
        long,
        short = 'q',
        required = false,
        takes_value = true,
        help = "Set the default quiet mode"
    )]
    default_quiet_mode: Option<bool>,
}

pub fn config_subcommand(config: Config) -> Result<()> {
    if let Some(quiet) = config.default_quiet_mode {
        config::set_quiet_mode(quiet)?;
    }
    Ok(())
}
