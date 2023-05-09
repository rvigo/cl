use super::Subcommand;
use crate::{
    command::Command,
    load_commands,
    resources::{config::Config, logger::interceptor::ErrorInterceptor},
};
use anyhow::Result;
use clap::Parser;
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
    fn run(&self, config: Config) -> Result<()> {
        let commands = load_commands!(config.get_command_file_path()).log_error()?;

        if self.description {
            if let Some(alias) = &self.alias {
                let namespace = &self.namespace;
                let command = commands.find_command(alias.clone(), namespace.to_owned())?;
                println!("{}", command.to_color_string());
            }
        } else if self.fzf {
            let mut seen = HashSet::with_capacity(commands.command_list().len());
            let duplicated: Vec<String> = commands
                .command_list()
                .iter()
                .filter_map(|c| {
                    if seen.contains(&c.alias) {
                        Some(c.alias.to_owned())
                    } else {
                        seen.insert(c.alias.to_owned());
                        None
                    }
                })
                .collect();

            commands.command_list().iter().for_each(|c| {
                if duplicated.contains(&c.alias) {
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
            commands
                .command_list()
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

impl ToColorString for Command {
    fn to_color_string(&self) -> String {
        format!(
            "Alias: {}\nNamespace: {}\nDescription: {}\nTags: {}\nCommand: {}",
            self.alias.fg::<CustomColor<201, 165, 249>>(),
            self.namespace.fg::<CustomColor<201, 165, 249>>(),
            self.description
                .as_ref()
                .unwrap_or(&String::default())
                .fg::<CustomColor<201, 165, 249>>(),
            self.tags_as_string().fg::<CustomColor<201, 165, 249>>(),
            self.command.fg::<CustomColor<201, 165, 249>>(),
        )
    }
}

trait Sumarize {
    /// Returns a sumarized string of `self`
    fn sumarize(&self) -> String;
}

impl Sumarize for Command {
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
            self.command.to_owned()
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

    #[test]
    fn should_format_a_command_with_description_to_string() {
        let command = Command {
            namespace: "namespace".to_owned(),
            command: "command".to_owned(),
            description: Some("description".to_owned()),
            alias: "alias".to_owned(),
            tags: None,
        };
        let expected_output = "namespace.alias: description --> command".to_owned();
        assert_eq!(command.sumarize(), expected_output);
    }

    #[test]
    fn should_format_a_command_without_description_to_string() {
        let command = Command {
            namespace: "namespace".to_owned(),
            command: "command".to_owned(),
            description: None,
            alias: "alias".to_owned(),
            tags: None,
        };
        let expected_output = "namespace.alias --> command".to_owned();
        assert_eq!(command.sumarize(), expected_output);
    }
}
