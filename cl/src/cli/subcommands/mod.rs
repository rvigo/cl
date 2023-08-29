pub mod config;
pub mod exec;
pub mod misc;
pub mod share;

use anyhow::Result;
use cl_core::resources::config::Config;

/// Represents a CLI Subcommand
pub trait Subcommand {
    /// Runs the subcommand with the given `Config`
    fn run(&self, config: Config) -> Result<()>;
}
