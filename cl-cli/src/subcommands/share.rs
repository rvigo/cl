use super::Subcommand;
use anyhow::{Context, Result};
use cl_core::{fs, initialize_commands, Command, CommandMapExt, CommandVecExt, Commands, Config};
use clap::{Parser, ValueEnum};
use std::{collections::HashSet, path::PathBuf};
use tracing::{debug, info, info_span, warn};

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
        let _span = info_span!("share::import", file = ?self.file_location).entered();
        let mut stored_commands = commands.as_list();
        let binding = fs::load_from(&self.file_location)?;
        let mut commands_from_file: Vec<Command> = binding.to_vec();

        let namespace_filter = self.create_namespace_filter();
        if !namespace_filter.is_empty() {
            debug!(target: "cl::share", namespaces = ?namespace_filter, "namespace filter applied");
        }
        commands_from_file.retain(|cmd| {
            namespace_filter.is_empty() || namespace_filter.contains(&cmd.namespace.to_string())
        });

        // Build duplicate key set and warn — block ensures the borrow on commands_from_file
        // is released before the mutable retain below.
        let duplicate_keys: HashSet<(String, String)> = {
            let duplicates = self.find_duplicates(&stored_commands, &commands_from_file);
            if !duplicates.is_empty() {
                warn!(
                    target: "cl::share",
                    "duplicated aliases found; they will be skipped:\n{}",
                    duplicates
                        .iter()
                        .map(|(alias, namespace)| format!("  - alias: {alias}, namespace: {namespace}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
            duplicates
                .into_iter()
                .map(|(a, ns)| (a.to_string(), ns.to_string()))
                .collect()
        };

        // Remove duplicates from commands to be imported using O(1) HashSet lookup
        commands_from_file.retain(|cmd| {
            !duplicate_keys.contains(&(cmd.alias.to_string(), cmd.namespace.to_string()))
        });

        if !commands_from_file.is_empty() {
            let count = commands_from_file.len();
            stored_commands.extend(commands_from_file);
            fs::save_at(
                &stored_commands.to_command_map(),
                config.command_file_path(),
            )
            .context("Could not import the aliases")?;
            info!(target: "cl::share", count, "aliases imported");
        } else {
            info!(target: "cl::share", "no aliases to import");
        }

        Ok(())
    }

    fn handle_export(&self, commands: &Commands) -> Result<()> {
        let _span = info_span!("share::export", file = ?self.file_location).entered();
        let namespace_filter = self.create_namespace_filter();
        if !namespace_filter.is_empty() {
            debug!(target: "cl::share", namespaces = ?namespace_filter, "namespace filter applied");
        }
        let filtered_commands: Vec<_> = commands
            .as_list()
            .into_iter()
            .filter(|cmd| {
                namespace_filter.is_empty() || namespace_filter.contains(&cmd.namespace.to_string())
            })
            .collect();

        fs::save_at(&filtered_commands.to_command_map(), &self.file_location)
            .context("Could not export the aliases")?;
        info!(target: "cl::share", count = filtered_commands.len(), "aliases exported");

        Ok(())
    }

    fn create_namespace_filter(&self) -> HashSet<String> {
        self.namespace
            .as_ref()
            .map_or(HashSet::new(), |namespaces| {
                namespaces.iter().cloned().collect()
            })
    }

    fn find_duplicates<'a>(
        &self,
        stored_commands: &'a [Command],
        new_commands: &'a [Command],
    ) -> Vec<(&'a str, &'a str)> {
        let existing_keys: HashSet<(&str, &str)> = stored_commands
            .iter()
            .map(|cmd| (cmd.alias.as_ref(), cmd.namespace.as_ref()))
            .collect();

        new_commands
            .iter()
            .filter(|cmd| existing_keys.contains(&(cmd.alias.as_ref(), cmd.namespace.as_ref())))
            .map(|cmd| (cmd.alias.as_ref(), cmd.namespace.as_ref()))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::borrow::Cow;

    fn cmd<'a>(alias: &'a str, namespace: &'a str) -> Command<'a> {
        Command {
            alias: Cow::Borrowed(alias),
            namespace: Cow::Borrowed(namespace),
            command: Cow::Borrowed("echo test"),
            description: None,
            tags: None,
        }
    }

    fn share() -> Share {
        Share {
            mode: Mode::Import,
            file_location: PathBuf::from("dummy.toml"),
            namespace: None,
        }
    }

    #[test]
    fn should_detect_duplicate_by_alias_and_namespace() {
        let s = share();
        let stored = vec![cmd("foo", "bar")];
        let incoming = vec![cmd("foo", "bar"), cmd("baz", "bar")];
        let duplicates = s.find_duplicates(&stored, &incoming);
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0].0, "foo");
    }

    #[test]
    fn should_not_flag_same_alias_in_different_namespace() {
        let s = share();
        let stored = vec![cmd("foo", "bar")];
        let incoming = vec![cmd("foo", "other")];
        let duplicates = s.find_duplicates(&stored, &incoming);
        assert!(duplicates.is_empty());
    }

    #[test]
    fn should_return_empty_when_no_duplicates() {
        let s = share();
        let stored = vec![cmd("foo", "bar")];
        let incoming = vec![cmd("baz", "bar")];
        let duplicates = s.find_duplicates(&stored, &incoming);
        assert!(duplicates.is_empty());
    }
}
