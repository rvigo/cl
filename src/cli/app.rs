use clap::{Parser, Subcommand};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser, Debug)]
#[clap(
    name = PKG_NAME,
    version,
    about,
    long_about = None,
    propagate_version = false,
    dont_collapse_args_in_usage = true,
    args_conflicts_with_subcommands = true
)]
pub struct App {
    #[clap(subcommand)]
    pub subcommand: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[clap(visible_aliases = &["X", "x"],
           about="Run your commands via CLI")]
    Exec(Exec),
}

#[derive(Parser, Debug)]
pub struct Exec {
    #[clap(required = true, help = "The alias to be executed")]
    pub alias: String,
    #[clap(
        required = false,
        requires = "alias",
        help = "The args of the alias command to be executed\n\
        Flags should be escaped with '\\' and surrounded by quotes\n   \
        e.g: cl exec <some-alias> '\\--flag'"
    )]
    pub args: Vec<String>,
    #[clap(
        short,
        long,
        requires = "alias",
        required = false,
        help = "The namespace in case of duplicated aliases"
    )]
    pub namespace: Option<String>,
    #[clap(
        short,
        long,
        action,
        help = "Dry run mode (Just prints the alias command in the terminal)"
    )]
    pub dry_run: bool,
    #[clap(
        short,
        long,
        action,
        help = "Quiet mode (Prints only the command execution)"
    )]
    pub quiet: bool,
    #[clap(
        multiple_values = true,
        last = true,
        requires = "alias",
        value_name = "NAMED PARAMETERS",
        help = "The command named parameters. Should be used after all args\n   \
            e.g: cl exec <some-alias> -- --named-parameter value"
    )]
    pub named_params: Vec<String>,
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
