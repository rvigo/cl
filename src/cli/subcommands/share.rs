use super::Subcommand;
use crate::{
    command::Command,
    commands::Commands,
    resources::{config::Config, file_service::FileService, logger::interceptor::ErrorInterceptor},
};
use anyhow::{Context, Result};
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

impl Subcommand for Share {
    fn run(&self, config: Config) -> Result<()> {
        let file_location = &self.file_location;
        let namespaces = &self.namespace;

        let file_service = FileService::new(config.get_command_file_path());
        let command_list = file_service.load_commands_from_file().log_error()?;
        let commands = Commands::init(command_list);

        match self.mode {
            Mode::Import => {
                let mut stored_commands = commands.command_list().to_owned();
                let mut commands_from_file: Vec<Command> = vec![];

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
                    file_service
                        .write_toml_file(&stored_commands, &config.get_command_file_path())
                        .context("Could not import the aliases")?;
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
                let mut command_list = Vec::default();
                if let Some(namespaces) = namespaces {
                    for namespace in namespaces.iter() {
                        command_list.append(
                            &mut commands
                                .command_list()
                                .iter()
                                .filter(|c| c.namespace.eq(namespace))
                                .map(|c| c.to_owned())
                                .collect(),
                        );
                    }
                } else {
                    command_list = commands.command_list().to_owned();
                }

                file_service
                    .write_toml_file(&command_list, file_location)
                    .context("Could not export the aliases")?;
                println!("Info: Exported {} aliases", command_list.len())
            }
        }
        Ok(())
    }
}
