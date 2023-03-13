use crate::{
    command::Command,
    commands::Commands,
    resources::{config::Config, file_service::FileService},
};
use anyhow::Result;
use clap::Parser;
use owo_colors::{colors::CustomColor, OwoColorize};

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

pub fn misc_subcommand(misc: Misc, config: Config) -> Result<()> {
    let command_list =
        FileService::new(config.get_command_file_path()?).load_commands_from_file()?;
    let commands = Commands::init(command_list);
    if misc.description {
        if let Some(alias) = misc.alias {
            let command = commands.find_command(alias, misc.namespace)?;
            print_colorized_command(command);
        }
    } else if misc.fzf {
        commands
            .command_list()
            .iter()
            .for_each(|c| println!("{}", c.alias))
    } else {
        commands
            .command_list()
            .iter()
            .for_each(|c| println!("{}", command_to_string(c.to_owned())));
    }

    Ok(())
}

fn command_to_string(command: Command) -> String {
    if let Some(desc) = command.description {
        format!(
            "{}.{}: {} --> {}",
            command.namespace,
            command.alias,
            desc,
            sanitize_string(command.command)
        )
    } else {
        format!(
            "{}.{} --> {}",
            command.namespace,
            command.alias,
            sanitize_string(command.command)
        )
    }
}

fn sanitize_string(command: String) -> String {
    let max_lenght_command: String = command.chars().take(50).collect();
    if max_lenght_command.contains('\n') {
        let idx = command.find('\n').unwrap_or(51);
        let short_command = format!("{}{}", &command[..(idx)], "...");
        short_command
    } else if max_lenght_command.len() == 50 {
        let short_command = format!("{}{}", &command[..50], "...");
        short_command
    } else {
        command
    }
}

fn print_colorized_command(command: Command) {
    println!(
        "Alias: {}\nNamespace: {}\nDescription: {}\nTags: {}\nCommand: {}",
        command.alias.fg::<CustomColor<201, 165, 249>>(),
        command.namespace.fg::<CustomColor<201, 165, 249>>(),
        command
            .description
            .as_ref()
            .unwrap_or(&String::default())
            .fg::<CustomColor<201, 165, 249>>(),
        command.tags_as_string().fg::<CustomColor<201, 165, 249>>(),
        command.command.fg::<CustomColor<201, 165, 249>>(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_format_a_command_with_description_to_string() {
        let command = Command {
            namespace: "namespace".to_string(),
            command: "command".to_string(),
            description: Some("description".to_string()),
            alias: "alias".to_string(),
            tags: None,
        };
        let expected_output = "namespace.alias: description --> command".to_string();
        assert_eq!(command_to_string(command), expected_output);
    }

    #[test]
    fn should_format_a_command_without_description_to_string() {
        let command = Command {
            namespace: "namespace".to_string(),
            command: "command".to_string(),
            description: None,
            alias: "alias".to_string(),
            tags: None,
        };
        let expected_output = "namespace.alias --> command".to_string();
        assert_eq!(command_to_string(command), expected_output);
    }

    #[test]
    fn should_sanitize_a_long_command_string() {
        let long_command = "a".repeat(60);
        let short_command = "a".repeat(40);

        assert_eq!(sanitize_string(long_command), "a".repeat(50) + "...");
        assert_eq!(sanitize_string(short_command.clone()), short_command);
    }

    #[test]
    fn should_sanitize_a_command_string_with_newline() {
        let input = "multiline command\n".to_string();
        let expected_output = "multiline command...".to_string();
        assert_eq!(sanitize_string(input), expected_output);
    }
}
