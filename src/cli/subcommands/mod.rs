pub(super) mod config;
pub(super) mod exec;
pub(super) mod misc;
pub(super) mod share;

use crate::resources::config::Config;
use anyhow::Result;

/// Represents a CLI Subcommand
pub trait Subcommand {
    /// Runs the subcommand with the given `Config`
    fn run(&self, config: Config) -> Result<()>;
}
