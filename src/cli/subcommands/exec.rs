use clap::Parser;

#[derive(Parser)]
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
