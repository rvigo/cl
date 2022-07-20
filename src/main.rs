mod cli;
mod command_file_service;
mod command_item;
mod commands;
mod configs;
mod gui;
mod utils;

use anyhow::Result;
use clap::ArgMatches;
use cli::app;
use command_file_service::CommandFileService;
use commands::Commands;
use gui::contexts::app::AppContext;

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
    app_context.callback_command()?;

    Ok(())
}

fn run_exec_command(sub_matches: &ArgMatches) -> Result<()> {
    let command_file_service = CommandFileService::init();
    let command_items = command_file_service.load_commands_from_file();
    let commands = Commands::init(command_items);

    let alias: String = sub_matches.value_of("alias").unwrap().into();
    let namespace = sub_matches
        .value_of("namespace")
        .map_or(None, |n| Some(String::from(n)));
    let args: Vec<String> = sub_matches
        .values_of("args")
        .unwrap_or(clap::Values::default())
        .map(String::from)
        .collect();

    let mut selected_command = commands.find_command(alias.clone(), namespace)?;
    selected_command.command = format!("{} {}", selected_command.command, &args.join(" "));
    commands.exec_command(&selected_command)?;

    Ok(())
}
