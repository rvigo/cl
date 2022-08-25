mod cli;
mod command;
mod commands;
mod gui;
mod resources;

use std::collections::HashMap;
use strfmt::strfmt;

use anyhow::{bail, Result};
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
    let named_args: Vec<String> = sub_matches
        .values_of("named")
        .expect("got nothing")
        .map(String::from)
        .collect();
    let command_items = file_service::load_commands_from_file()?;
    let commands = Commands::init(command_items);

    let alias: String = sub_matches.value_of("alias").unwrap().into();
    let namespace = sub_matches.get_one::<String>("namespace").map(String::from);
    let args: Vec<String> = sub_matches
        .values_of("args")
        .unwrap_or_default()
        .map(String::from)
        .collect();

    let mut selected_command = commands.find_command(alias, namespace)?;
    if selected_command.command.contains('#') {
        let mut mapped = HashMap::<String, String>::new();
        for i in named_args {
            if i.starts_with("--") {
                if i.contains("=") {
                    let mut values = i.split("=");
                    let key = values.next().unwrap().to_string().replace("--", "");
                    let value = values.next().unwrap().to_string();
                    mapped.insert(key, value);
                    continue;
                }

                let key = i.replace("--", "");
                mapped.insert(key, String::default());
            } else {
                let key = mapped
                    .iter()
                    .find(|(key, value)| !key.is_empty() && value.is_empty())
                    .unwrap()
                    .0;
                mapped.insert(key.to_string(), i);
            }
        }
        println!("{mapped:?}");
        selected_command.command = selected_command.command.replace("#", "");
        selected_command.command = if let Ok(command) = strfmt(&selected_command.command, &mapped) {
            command
        } else {
            bail!("invalid named arguments!!!!")
        }
    } else {
        selected_command.command = format!("{} {}", selected_command.command, &args.join(" "));
    }
    commands.exec_command(&selected_command)
}
