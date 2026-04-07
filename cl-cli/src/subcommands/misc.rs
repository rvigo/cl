use super::Subcommand;
use anyhow::{bail, Result};
use cl_core::{initialize_commands, Command, Config};
use clap::Parser;
use itertools::Itertools;
use owo_colors::{colors::CustomColor, OwoColorize};
use std::collections::HashSet;

#[derive(Parser)]
pub struct Misc {
    #[clap(short, action, required = false)]
    description: bool,
    #[clap(short, required = false)]
    alias: Option<String>,
    #[clap(short, required = false)]
    namespace: Option<String>,
    #[clap(short, action, required = false)]
    fzf: bool,
}

impl Subcommand for Misc {
    fn run(&self, config: impl Config) -> Result<()> {
        let commands = initialize_commands!(config.command_file_path());
        let command_vec = commands.as_list();
        let sorted_commands = command_vec
            .iter()
            .sorted_by_key(|c| c.alias.clone())
            .collect::<Vec<&Command>>();

        if self.description {
            let Some(alias) = &self.alias else {
                bail!("--alias/-a is required when using --description/-d");
            };
            let namespace = &self.namespace;
            let command = commands.find(alias, namespace.as_deref())?;
            println!("{}", command.to_color_string());
        } else if self.fzf {
            let mut seen = HashSet::with_capacity(command_vec.len());
            let duplicated: HashSet<String> = command_vec
                .iter()
                .filter_map(|c| {
                    if seen.contains(&c.alias.clone()) {
                        Some(c.alias.to_string())
                    } else {
                        seen.insert(c.alias.clone());
                        None
                    }
                })
                .collect();

            sorted_commands.iter().for_each(|c| {
                if duplicated.contains(&c.alias.to_string()) {
                    println!(
                        "{} ({})",
                        c.alias,
                        c.namespace.fg::<CustomColor<201, 165, 249>>()
                    )
                } else {
                    println!("{}", c.alias)
                }
            })
        } else {
            sorted_commands
                .iter()
                .for_each(|c| println!("{}", c.summarize()));
        }

        Ok(())
    }
}

trait ToColorString {
    /// Returns a colored String representation of `self`
    fn to_color_string(&self) -> String;
}

impl ToColorString for Command<'_> {
    fn to_color_string(&self) -> String {
        format!(
            "Alias: {}\nNamespace: {}\nDescription: {}\nTags: {}\nCommand: {}",
            self.alias.fg::<CustomColor<201, 165, 249>>(),
            self.namespace.fg::<CustomColor<201, 165, 249>>(),
            self.description().fg::<CustomColor<201, 165, 249>>(),
            self.tags_as_string().fg::<CustomColor<201, 165, 249>>(),
            self.command.fg::<CustomColor<201, 165, 249>>(),
        )
    }
}

trait Summarize {
    /// Returns a summarized string of `self`
    fn summarize(&self) -> String;
}

const MAX_COMMAND_LEN: usize = 50;

impl Summarize for Command<'_> {
    fn summarize(&self) -> String {
        let command_string = &self.command;
        let max_length_command: String = command_string.chars().take(MAX_COMMAND_LEN).collect();
        let command_string = if max_length_command.contains('\n') {
            let idx = self.command.find('\n').expect("newline confirmed in preview");
            format!("{}...", &self.command[..idx])
        } else if max_length_command.len() == MAX_COMMAND_LEN {
            format!("{}...", &self.command[..MAX_COMMAND_LEN])
        } else {
            self.command.to_string()
        };

        if let Some(ref desc) = self.description {
            format!(
                "{}.{}: {} --> {}",
                self.namespace, self.alias, desc, command_string
            )
        } else {
            format!("{}.{} --> {}", self.namespace, self.alias, command_string)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cl_core::Preferences;
    use clap::Parser;
    use std::borrow::Cow;
    use std::path::PathBuf;

    struct MockConfig {
        commands_file: PathBuf,
        preferences: Preferences,
    }

    impl cl_core::Config for MockConfig {
        fn load() -> anyhow::Result<Self>
        where
            Self: Sized,
        {
            unimplemented!()
        }
        fn save(&self) -> anyhow::Result<()> {
            Ok(())
        }
        fn preferences(&self) -> &Preferences {
            &self.preferences
        }
        fn preferences_mut(&mut self) -> &mut Preferences {
            &mut self.preferences
        }
        fn command_file_path(&self) -> PathBuf {
            self.commands_file.clone()
        }
        fn log_dir_path(&self) -> anyhow::Result<PathBuf> {
            unimplemented!()
        }
    }

    fn mock_config() -> MockConfig {
        let path = std::env::temp_dir().join("cl_test_misc_commands.toml");
        std::fs::write(&path, "").unwrap();
        MockConfig {
            commands_file: path,
            preferences: Preferences::default(),
        }
    }

    #[test]
    fn should_return_error_when_description_flag_used_without_alias() {
        let misc = Misc::parse_from(["cl", "-d"]);
        let result = misc.run(mock_config());
        assert!(result.is_err());
        assert!(
            result.unwrap_err().to_string().contains("--alias/-a is required"),
            "Error should tell user to provide --alias/-a"
        );
    }

    #[test]
    fn should_format_a_command_with_description_to_string() {
        let command = Command {
            namespace: Cow::Borrowed("namespace"),
            command: Cow::Borrowed("command"),
            description: Some(Cow::Borrowed("description")),
            alias: Cow::Borrowed("alias"),
            tags: None,
        };
        let expected_output = "namespace.alias: description --> command".to_owned();
        assert_eq!(command.summarize(), expected_output);
    }

    #[test]
    fn should_format_a_command_without_description_to_string() {
        let command = Command {
            namespace: Cow::Borrowed("namespace"),
            command: Cow::Borrowed("command"),
            description: None,
            alias: Cow::Borrowed("alias"),
            tags: None,
        };
        let expected_output = "namespace.alias --> command".to_owned();
        assert_eq!(command.summarize(), expected_output);
    }

    #[test]
    fn should_truncate_command_longer_than_50_chars() {
        let long_cmd = "a".repeat(60);
        let command = Command {
            namespace: Cow::Borrowed("ns"),
            command: Cow::Owned(long_cmd),
            description: None,
            alias: Cow::Borrowed("al"),
            tags: None,
        };
        let result = command.summarize();
        assert!(
            result.ends_with("..."),
            "Long command should be truncated with '...'"
        );
        // 50 chars of content + "..." = 53 chars after the arrow
        let cmd_part = result.split(" --> ").nth(1).unwrap();
        assert_eq!(cmd_part.len(), 53);
    }

    #[test]
    fn should_truncate_multiline_command_at_first_newline() {
        let command = Command {
            namespace: Cow::Borrowed("ns"),
            command: Cow::Borrowed("first line\nsecond line"),
            description: None,
            alias: Cow::Borrowed("al"),
            tags: None,
        };
        let result = command.summarize();
        assert!(result.ends_with("first line..."));
    }
}
