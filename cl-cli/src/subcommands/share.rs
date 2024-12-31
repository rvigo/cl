use super::Subcommand;
use anyhow::{Context, Result};
use cl_core::{fs, initialize_commands, Command, CommandMapExt, CommandVecExt, Commands, Config};
use clap::{Parser, ValueEnum};
use log::{info, warn};
use std::{borrow::Cow, collections::HashSet, path::PathBuf};

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
    fn run(&self, config: impl Config) -> Result<()> {
        let commands = initialize_commands!(config.command_file_path());

        match self.mode {
            Mode::Import => self.handle_import(&commands, config),
            Mode::Export => self.handle_export(&commands),
        }
    }
}
impl Share {
    fn handle_import(&self, commands: &Commands, config: impl Config) -> Result<()> {
        let mut stored_commands = commands.as_list();
        let binding = fs::load_from(&self.file_location)?;
        let mut commands_from_file: Vec<Command> = binding.to_vec();

        let namespace_filter = self.create_namespace_filter();
        commands_from_file.retain(|cmd| {
            namespace_filter.is_empty() || namespace_filter.contains(&cmd.namespace.to_string())
        });

        let duplicates = self.find_duplicates(&stored_commands, &commands_from_file);
        if !duplicates.is_empty() {
            warn!(
                "Duplicated aliases found! Please adjust them:\n{}",
                duplicates
                    .iter()
                    .map(|cmd| format!(" - alias: {}, namespace: {}", cmd.alias, cmd.namespace))
                    .collect::<Vec<_>>()
                    .join(",\n")
            );
        }

        // Remove duplicates from commands to be imported
        commands_from_file.retain(|cmd| {
            !stored_commands
                .iter()
                .any(|stored| cmd.alias == stored.alias && cmd.namespace == stored.namespace)
        });

        if !commands_from_file.is_empty() {
            stored_commands.extend(commands_from_file.clone());
            fs::save_at(
                &stored_commands.to_command_map(),
                config.command_file_path(),
            )
            .context("Could not import the aliases")?;
            info!("Successfully imported {} aliases", commands_from_file.len());
        } else {
            info!("There are no aliases to be imported");
        }

        Ok(())
    }

    fn handle_export(&self, commands: &Commands) -> Result<()> {
        let filtered_commands = if let Some(namespaces) = &self.namespace {
            commands
                .as_list()
                .into_iter()
                .filter(|cmd| namespaces.contains(&cmd.namespace.to_string()))
                .collect()
        } else {
            commands.as_list()
        };

        fs::save_at(&filtered_commands.to_command_map(), &self.file_location)
            .context("Could not export the aliases")?;
        info!("Exported {} aliases", filtered_commands.len());

        Ok(())
    }

    fn create_namespace_filter(&self) -> HashSet<String> {
        self.namespace.clone().map_or(HashSet::new(), |namespaces| {
            namespaces.into_iter().collect()
        })
    }

    fn find_duplicates<'a>(
        &'a self,
        stored_commands: &'a [Command],
        new_commands: &'a [Command],
    ) -> Vec<Cow<'a, Command>> {
        let existing_keys: HashSet<_> = stored_commands
            .iter()
            .map(|cmd| (Cow::Borrowed(&cmd.alias), Cow::Borrowed(&cmd.namespace)))
            .collect();

        new_commands
            .iter()
            .filter(|cmd| {
                existing_keys.contains(&(Cow::Borrowed(&cmd.alias), Cow::Borrowed(&cmd.namespace)))
            })
            .map(Cow::Borrowed)
            .collect()
    }
}
