mod cli;
mod command;
mod commands;
mod gui;
mod resources;

use anyhow::Result;
use clap::ArgMatches;
use cli::app;
use commands::Commands;
use gui::entities::app::AppContext;
use resources::file_service;

fn main() -> Result<()> {
    let app = app::build_app();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("X", sub_matches)) => run_exec_command(sub_matches),
        _ => run_main_app(),
    }
}

fn run_main_app() -> Result<()> {
    let mut app_context = AppContext::create()?;
    app_context.render()?;
    app_context.clear()?;

    app_context.callback_command()
}

fn run_exec_command(sub_matches: &ArgMatches) -> Result<()> {
    let command_items = file_service::load_commands_from_file()?;
    let commands = Commands::init(command_items);

    let alias: String = sub_matches.value_of("alias").unwrap().into();
    let namespace = sub_matches.value_of("namespace").map(String::from);
    let args: Vec<String> = sub_matches
        .values_of("args")
        .unwrap_or_default()
        .map(String::from)
        .collect();

    let mut selected_command = commands.find_command(alias, namespace)?;
    selected_command.command = format!("{} {}", selected_command.command, &args.join(" "));

    commands.exec_command(&selected_command)
}
