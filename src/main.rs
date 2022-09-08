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
    let app = App::parse();

    match app.subcommand {
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
    let quiet_mode: bool = exec.quiet;

    let mut command_item = commands.find_command(alias, namespace)?;
    command_item.command = cli::utils::prepare_command(command_item.command, named_args, args)?;
    commands.exec_command(&command_item, dry_run, quiet_mode)
}
