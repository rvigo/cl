use crate::{cli, resources};
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Exec {
    #[clap(required = true, help = "The alias to be executed")]
    alias: String,
    #[clap(
        required = false,
        requires = "alias",
        help = "The args of the alias command to be executed\n\
        Flags should be escaped with '\\' and surrounded by quotes\n   \
        e.g: cl exec <some-alias> '\\--flag'"
    )]
    args: Vec<String>,
    #[clap(
        short,
        long,
        requires = "alias",
        required = false,
        help = "The namespace in case of duplicated aliases"
    )]
    namespace: Option<String>,
    #[clap(
        short,
        long,
        action,
        help = "Dry run mode (Just prints the alias command in the terminal)"
    )]
    dry_run: bool,
    #[clap(
        short,
        long,
        action,
        help = "Quiet mode (Prints only the command execution)"
    )]
    quiet: bool,
    #[clap(
        num_args(1..),
        last = true,
        requires = "alias",
        value_name = "NAMED PARAMETERS",
        help = "The command named parameters. Should be used after all args\n   \
            e.g: cl exec <some-alias> -- --named-parameter value"
    )]
    named_params: Vec<String>,
}

pub fn exec_subcommand(exec: Exec) -> Result<()> {
    let commands = resources::load_commands()?;

    let alias: String = exec.alias;
    let namespace: Option<String> = exec.namespace;
    let args: Vec<String> = exec.args;
    let named_args: Vec<String> = exec.named_params;
    let dry_run: bool = exec.dry_run;
    let quiet_mode: bool = exec.quiet;

    let mut command_item = commands.find_command(alias, namespace)?;
    command_item.command = cli::utils::prepare_command(command_item.command, named_args, args)?;
    commands.exec_command(&command_item, dry_run, quiet_mode)
}
