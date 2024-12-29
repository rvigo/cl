mod add;
mod config;
mod exec;
mod misc;
mod share;

pub use add::Add;
pub use config::Config;
pub use exec::Exec;
pub use misc::Misc;
pub use share::Share;

use anyhow::Result;
use cl_core::Config as CoreConfig;

/// Represents a CLI Subcommand
pub trait Subcommand {
    /// Runs the subcommand with the given `Config`
    fn run(&self, config: CoreConfig) -> Result<()>;
}
