use anyhow::Result;
use app::Subcommands;
use cl_core::Config;
use subcommands::Subcommand;

pub mod app;
pub mod subcommands;

pub fn run_subcommands(subcommands: Subcommands, config: Config) -> Result<()> {
    match subcommands {
        Subcommands::Exec(exec) => exec.run(config),
        Subcommands::Share(share) => share.run(config),
        Subcommands::Misc(misc) => misc.run(config),
        Subcommands::Config(subcommand_config) => subcommand_config.run(config),
    }
}
