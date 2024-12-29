use super::subcommands::{Add, Config, Exec, Misc, Share};
use clap::{Parser, Subcommand as ClapSubcommand};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[clap(
    name = PKG_NAME,
    about,
    long_about = None,
    version = PKG_VERSION,
    propagate_version = false,
    dont_collapse_args_in_usage = true,
    args_conflicts_with_subcommands = true,
)]
pub struct App {
    #[clap(subcommand)]
    pub subcommands: Option<Subcommands>,
}

impl App {
    pub fn parse_app() -> App {
        App::parse()
    }
}

#[derive(ClapSubcommand)]
pub enum Subcommands {
    #[clap(aliases = &["X", "x"],
    about="Run your commands via CLI")]
    Exec(Exec),
    #[clap(aliases = &["S", "s"],
    about = "Import/Export aliases")]
    Share(Share),
    #[clap(about = "Configure your app")]
    Config(Config),
    #[clap(hide = true)]
    // this subcommand should not be visible
    Misc(Misc),
    #[clap(about = "Add your command via cli")]
    Add(Add),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        App::command().debug_assert()
    }
}
