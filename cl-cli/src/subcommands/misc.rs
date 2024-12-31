use super::Subcommand;
use anyhow::Result;
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
            if let Some(alias) = &self.alias {
                let namespace = &self.namespace;
                let command = commands.find(alias, namespace.as_deref())?;
                println!("{}", command.to_color_string());
            }
        } else if self.fzf {
            let mut seen = HashSet::with_capacity(command_vec.len());
            let duplicated: Vec<String> = command_vec
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
                .for_each(|c| println!("{}", c.sumarize()));
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

trait Sumarize {
    /// Returns a sumarized string of `self`
    fn sumarize(&self) -> String;
}

// TODO wtf?
impl Sumarize for Command<'_> {
    fn sumarize(&self) -> String {
        let command_string = &self.command;
        let max_lenght_command: String = command_string.chars().take(50).collect();
        let command_string = if max_lenght_command.contains('\n') {
            let idx = self.command.find('\n').unwrap_or(51);
            let short_command = format!("{}{}", &self.command[..(idx)], "...");
            short_command
        } else if max_lenght_command.len() == 50 {
            let short_command = format!("{}{}", &self.command[..50], "...");
            short_command
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
    use std::borrow::Cow;

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
        assert_eq!(command.sumarize(), expected_output);
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
        assert_eq!(command.sumarize(), expected_output);
    }
}
