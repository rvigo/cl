use super::Subcommand;
use crate::{
    command::Command,
    commands::Commands,
    resources::{
        commands_file_service::CommandsFileService, config::Config,
        logger::interceptor::ErrorInterceptor,
    },
};
use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use log::{info, warn};
use std::{collections::HashSet, path::PathBuf};

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

        let commands_file_service =
            CommandsFileService::new(config.get_command_file_path()).validate()?;
        let command_list = commands_file_service.load().log_error()?;
        let commands = Commands::init(command_list);

        match self.mode {
            Mode::Import => {
                let mut stored_commands = commands.command_list().to_owned();
                let commands_from_file: Vec<Command> =
                    commands_file_service.load_from(&self.file_location)?;

                let namespaces_set: HashSet<String> =
                    namespaces.to_owned().map_or(HashSet::new(), |n| {
                        n.into_iter().collect::<HashSet<String>>()
                    });

                // filter by namespaces
                let mut commands_from_file: Vec<Command> = commands_from_file
                    .into_iter()
                    .filter(|c| namespaces_set.is_empty() || namespaces_set.contains(&c.namespace))
                    .collect();

                // duplicated items
                let mut duplicates = Vec::new();
                let mut reference_set = HashSet::new();

                for command in &stored_commands {
                    reference_set.insert((&command.alias, &command.namespace));
                }

                for command in commands_from_file.iter().cloned() {
                    let key = (&command.alias, &command.namespace);
                    if reference_set.contains(&key) {
                        duplicates.push(command)
                    }
                }

                if !duplicates.is_empty() {
                    warn!(
                        "Duplicated aliases found! Please adjust them by choosing a new alias/namespace:\n{}",
                            duplicates.iter()
                            .map(|item| format!(" - alias: {}, namespace: {}", item.alias.clone(), item.namespace.clone()))
                            .collect::<Vec<_>>()
                            .join(",\n")
                    );
                }

                // remove duplicates
                commands_from_file.retain(|c| {
                    !stored_commands
                        .iter()
                        .any(|s| c.alias == s.alias && c.namespace == s.namespace)
                });

                if !commands_from_file.is_empty() {
                    stored_commands.extend(commands_from_file.to_owned());
                    commands_file_service
                        .save(&stored_commands)
                        .context("Could not import the aliases")?;
                    info!(
                        "Successfully imported {} aliases",
                        commands_from_file.len() - duplicates.len()
                    )
                } else {
                    info!("There are no aliases to be imported");
                }
            }
            Mode::Export => {
                info!("Exporting aliases to: {}", file_location.display());
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

                commands_file_service
                    .save_at(&command_list, file_location)
                    .context("Could not export the aliases")?;
                info!("Exported {} aliases", command_list.len())
            }
        }
        Ok(())
    }
}
