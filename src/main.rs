mod cli;
mod command;
mod commands;
mod gui;
mod resources;

use anyhow::Result;
use clap::Parser;
use cli::app::{App, Exec, SubCommand};
use commands::Commands;
use gui::entities::app::AppContext;
use resources::file_service;

fn main() -> Result<()> {
    let cli = App::parse();

    match cli.subcommand {
        Some(SubCommand::Exec(exec)) => run_command(exec),
        _ => run_main_app(),
    }
}

fn run_main_app() -> Result<()> {
    let mut app_context = AppContext::create()?;
    app_context.render()?;
    app_context.clear()?;

    app_context.callback_command()
}

fn run_command(exec: Exec) -> Result<()> {
    let command_items = file_service::load_commands_from_file()?;
    let commands = Commands::init(command_items);

    let alias: String = exec.alias;
    let namespace: Option<String> = exec.namespace;
    let args: Vec<String> = exec.args;

    let named_args: Vec<String> = exec.named_params;
    let dry_run: bool = exec.dry_run;
    let mut selected_command = commands.find_command(alias, namespace)?;
    selected_command.command =
        cli::utils::prepare_command(selected_command.command, named_args, args)?;
    commands.exec_command(&selected_command, dry_run)
}
