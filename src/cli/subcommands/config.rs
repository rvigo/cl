use super::Subcommand;
use crate::resources::config::{Config as AppConfig, LogLevel as ConfigLogLevel};
use anyhow::{bail, Result};
use clap::{Parser, Subcommand as ClapSubcommand, ValueEnum};
use std::{env, path::PathBuf, process::Command};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum LogLevel {
    Debug,
    Info,
    Error,
}

impl LogLevel {
    pub fn as_config_enum(&self) -> ConfigLogLevel {
        match self {
            LogLevel::Debug => ConfigLogLevel::Debug,
            LogLevel::Info => ConfigLogLevel::Info,
            LogLevel::Error => ConfigLogLevel::Error,
        }
    }
}

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
    #[clap(
        value_parser,
        long,
        short = 'l',
        ignore_case = true,
        required = false,
        num_args(1),
        help = "Set the default log level"
    )]
    default_log_level: Option<LogLevel>,
    #[clap(
        long,
        short = 'H',
        required = false,
        num_args(1),
        help = "Set the `highlight matches` mode"
    )]
    highlight_matches: Option<bool>,
    #[clap(subcommand)]
    subcommand: Option<ConfigSubcommand>,
}

#[derive(ClapSubcommand)]
pub enum ConfigSubcommand {
    ZshWidget(Widget),
}

#[derive(Parser)]
pub struct Widget {
    #[clap(short, long, action, required = true, help = "Install the cl widget")]
    install: bool,
}

impl Subcommand for Config {
    fn run(&self, mut config: AppConfig) -> Result<()> {
        if let Some(ConfigSubcommand::ZshWidget(_)) = self.subcommand {
            install_zsh_widget(config.get_app_home_dir())?
        } else if let Some(quiet) = self.default_quiet_mode {
            config.set_default_quiet_mode(quiet)?;
            println!("quiet mode set to {quiet}")
        } else if let Some(log_level) = self.default_log_level {
            config.set_log_level(log_level.as_config_enum())?;
            println!("log level set to {log_level:?}")
        } else if let Some(highlight) = self.highlight_matches {
            config.set_highlight(highlight)?;
            println!("highlight matches set to {highlight}")
        } else {
            println!("{}", config.printable_string())
        }

        Ok(())
    }
}

fn install_zsh_widget(app_home_dir: PathBuf) -> Result<()> {
    if let Ok(shell) = env::var("SHELL") {
        if !shell.contains("zsh") {
            bail!("Cannot install zsh widget on non zsh shell! Actual $SHELL value is {shell}")
        }
    }

    validate_fzf()?;

    let widget = include_str!("../resources/zsh/cl-exec-widget");
    let dest_location = app_home_dir.join("cl-exec-widget");
    let create_file = format!("echo \'{widget}\' >> {}", dest_location.display());
    let source_file = format!("echo \"source {}\" >> ~/.zshrc", dest_location.display());

    run_shell(&create_file)?;
    run_shell(&source_file)?;

    println!("Done!!! Please restart your terminal and press CTRL+O to access the widget");

    Ok(())
}

fn validate_fzf() -> Result<()> {
    let output = Command::new("command").args(["-v", "fzf"]).output()?;

    if !output.status.success() {
        bail!("This widget needs fzf to work. Please first install it and then reinstall de widget")
    } else {
        Ok(())
    }
}

fn run_shell(command: &str) -> Result<()> {
    Command::new("zsh").arg("-c").arg(command).spawn()?;
    Ok(())
}
