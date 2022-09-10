mod cli;
mod command;
mod commands;
mod gui;
mod resources;

use anyhow::Result;
use clap::Parser;
use cli::{
    app::{App, SubCommand},
    subcommands::{
        exec::Exec,
        share::{Mode, Share},
    },
};
use gui::entities::app::AppContext;
use itertools::Itertools;
use resources::{config::CONFIG, file_service};
use std::path::PathBuf;

use crate::commands::Commands;

fn main() -> Result<()> {
    let app = App::parse();

    match app.subcommand {
        Some(SubCommand::Exec(exec)) => exec_subcommand(exec),
        Some(SubCommand::Share(share)) => share_subcommand(share),
        Some(SubCommand::All(_)) => all_subcommand(),
        _ => run_main_app(),
    }
}

fn all_subcommand() -> Result<()> {
    let commands = resources::load_commands()?;
    commands.into_iter().for_each(|c| {
        let command = if c.command.len() > 50 {
            format!("{}{}", &c.command[..50], "...")
        } else {
            c.command
        };
        if let Some(desc) = c.description {
            println!("{}.{}: {} --> {}", c.namespace, c.alias, desc, &command)
        } else {
            println!("{}.{} --> {}", c.namespace, c.alias, &command)
        }
    });
    Ok(())
}

fn share_subcommand(share: Share) -> Result<()> {
    let file_location: PathBuf = share.file_location;
    let namespaces = share.namespace;
    let commands = resources::load_commands()?;
    match share.mode {
        Mode::Import => {
            let mut stored_commands = commands.commands(String::from("All"), String::default())?;
            let mut commands_from_file = file_service::convert_from_toml_file(&file_location)?;

            //filter given namespaces
            if namespaces.is_some() {
                commands_from_file.retain(|item| {
                    namespaces
                        .as_ref()
                        .unwrap()
                        .iter()
                        .contains(&item.namespace)
                });
            }

            let reference_commands = commands_from_file.clone();

            //removes duplicated items
            commands_from_file.retain(|item| !stored_commands.contains(item));

            //get duplicated items using the reference_commands vec
            let diff: Vec<_> = reference_commands
                .iter()
                .filter(|item| !commands_from_file.contains(item))
                .collect();

            if !diff.is_empty() {
                eprintln!(
                    "Warning: Duplicated aliases found! Please adjust them by choosing a new alias/namespace:\n{}",
                    diff
                        .clone()
                        .iter()
                        .map(|item| format!(" - alias: {}, namespace: {}", item.alias.clone(), item.namespace.clone()))
                        .collect_vec()
                        .join(",\n")
                );
            }
            if !commands_from_file.is_empty() {
                stored_commands.append(&mut commands_from_file);
                file_service::write_toml_file(&stored_commands, &CONFIG.get_command_file_path())?;
                println!(
                    "Done: Successfully imported {} aliases",
                    commands_from_file.len()
                )
            } else {
                println!("Done: There are no aliases to be imported")
            }
        }
        Mode::Export => {
            eprintln!("Exporting aliases to: {}", file_location.display());
            let mut command_list = Commands::default();
            if let Some(namespaces) = namespaces {
                for namespace in namespaces.into_iter() {
                    command_list
                        .append(&mut commands.commands(namespace, String::default())?.to_vec());
                }
            } else {
                command_list = commands.commands(String::from("All"), String::default())?;
            }

            file_service::write_toml_file(&command_list, &file_location)?;
            println!("Done. Exported {} aliases", command_list.len())
        }
    }
    Ok(())
}

fn run_main_app() -> Result<()> {
    let mut app_context = AppContext::create()?;
    app_context.render()?;
    app_context.clear()?;

    app_context.callback_command()
}

fn exec_subcommand(exec: Exec) -> Result<()> {
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
