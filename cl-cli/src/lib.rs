use anyhow::Result;
use app::Subcommands;
use cl_core::Config;

pub mod app;
pub mod subcommands;

pub fn run_subcommands(subcommands: Subcommands, config: Config) -> Result<()> {
    subcommands.inner().run(config)
}
