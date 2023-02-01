use crate::{
    commands::Commands,
    resources::{self, config::CONFIG, file_service},
};
use anyhow::Result;
use clap::{Parser, ValueEnum};
use itertools::Itertools;
use std::path::PathBuf;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Export,
    Import,
}

#[derive(Parser)]
pub struct Share {
    #[clap(value_parser, help = "Export/Import mode")]
    mode: Mode,
    #[clap(
        short,
        long = "file_location",
        required = false,
        default_value = "shared.toml",
        hide_default_value = true,
        help = "If <MODE> is `export`, the location of the output file\n\
        If `import`, the location of the source file\n\
        (Default value is the current directory)",
        value_parser
    )]
    file_location: PathBuf,
    #[clap(
        short,
        long,
        num_args(1..),
        help = "The namespace(s) to be imported from/exported to file\nIf none, all aliases will be processed"
    )]
    namespace: Option<Vec<String>>,
}

pub fn share_subcommand(share: Share) -> Result<()> {
    let file_location: PathBuf = share.file_location;
    let namespaces = share.namespace;
    let commands = resources::load_commands()?;
    match share.mode {
        Mode::Import => {
            let mut stored_commands = commands.commands(String::from("All"), String::default())?;
            let mut commands_from_file = file_service::convert_from_toml_file(&file_location)?;

            //filter given namespaces
            if let Some(namespaces) = namespaces {
                commands_from_file.retain(|item| namespaces.contains(&item.namespace));
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
                    "Info: Successfully imported {} aliases",
                    commands_from_file.len()
                )
            } else {
                println!("There are no aliases to be imported")
            }
        }
        Mode::Export => {
            eprintln!("Exporting aliases to: {}", file_location.display());
            let mut command_list = Commands::default();
            if let Some(namespaces) = namespaces {
                for namespace in namespaces {
                    command_list
                        .append(&mut commands.commands(namespace, String::default())?.to_vec());
                }
            } else {
                command_list = commands.commands(String::from("All"), String::default())?;
            }

            file_service::write_toml_file(&command_list, &file_location)?;
            println!("Info: Exported {} aliases", command_list.len())
        }
    }
    Ok(())
}
