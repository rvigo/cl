use crate::resources::config;
use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use std::{env, process::Command};

#[derive(Parser)]
pub struct Config {
    #[clap(
        long,
        short = 'q',
        required = false,
        num_args(1),
        help = "Set the default quiet mode"
    )]
    default_quiet_mode: Option<bool>,
    #[clap(subcommand)]
    subcommand: Option<ConfigSubcommand>,
}

#[derive(Subcommand)]
pub enum ConfigSubcommand {
    ZshWidget(Widget),
}

#[derive(Parser)]

pub struct Widget {
    #[clap(short, long, action, required = true, help = "Install the cl widget")]
    install: bool,
}

pub fn config_subcommand(config: Config) -> Result<()> {
    if let Some(ConfigSubcommand::ZshWidget(_)) = config.subcommand {
        install_zsh_widget()?
    }
    if let Some(quiet) = config.default_quiet_mode {
        config::set_quiet_mode(quiet)?;
    }
    Ok(())
}

fn install_zsh_widget() -> Result<()> {
    if let Ok(shell) = env::var("SHELL") {
        if !shell.contains("zsh") {
            bail!("Cannot install zsh widget on non zsh shell! Actual $SHELL value is {shell}")
        }
    }
    let widget = include_str!("../resources/zsh/cl-exec-widget");
    let dest_location = "~/.config/cl/cl-exec-widget";
    let create_file = format!("echo \'{}\' >> {}", widget, dest_location);
    let source_file = format!("echo \"source {}\" >> ~/.zshrc", dest_location);

    run_shell(&create_file)?;
    run_shell(&source_file)?;

    println!("Done!!! Please restart your terminal and press CTRL+O to access the widget");

    Ok(())
}

fn run_shell(command: &str) -> Result<()> {
    Command::new("zsh").arg("-c").arg(command).spawn()?;
    Ok(())
}
