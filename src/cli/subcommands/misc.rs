use crate::{command::Command, resources};
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
    if command.len() > 50 {
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

pub fn misc_subcommand(misc: Misc) -> Result<()> {
    let commands = resources::load_commands()?;
    if misc.description {
        let command = commands.find_command(misc.alias.unwrap(), misc.namespace)?;
        print_colorized_command(command);
    } else if misc.fzf {
        commands.into_iter().for_each(|c| println!("{}", c.alias))
    } else {
        commands
            .into_iter()
            .for_each(|c| println!("{}", command_to_string(c)));
    }

    Ok(())
}
