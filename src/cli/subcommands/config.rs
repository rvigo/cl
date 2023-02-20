use crate::resources::config::Config as AppConfig;
use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use std::{env, process::Command, sync::Mutex};

lazy_static! {
    static ref APP_CONFIG: Mutex<AppConfig> = Mutex::new(
        AppConfig::load()
            .context("Cannot properly load the app configs")
            .unwrap()
    );
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
        APP_CONFIG.lock().unwrap().set_default_quiet_mode(quiet)?
    }
    Ok(())
}

fn install_zsh_widget() -> Result<()> {
    if let Ok(shell) = env::var("SHELL") {
        if !shell.contains("zsh") {
            bail!("Cannot install zsh widget on non zsh shell! Actual $SHELL value is {shell}")
        }
    }

    validate_fzf();

    let widget = include_str!("../resources/zsh/cl-exec-widget");
    let app_home_dir = APP_CONFIG.lock().unwrap().get_app_home_dir();
    let dest_location = format!("{}/cl-exec-widget", app_home_dir.display());
    let create_file = format!("echo \'{widget}\' >> {dest_location}");
    let source_file = format!("echo \"source {dest_location}\" >> ~/.zshrc");

    run_shell(&create_file)?;
    run_shell(&source_file)?;

    println!("Done!!! Please restart your terminal and press CTRL+O to access the widget");

    Ok(())
}

fn validate_fzf() {
    if let Ok(res) = Command::new("zsh")
        .arg("-c")
        .arg("$(command -v fzf)")
        .output()
    {
        if !res.status.success() {
            eprintln!("Please first install fzf and then install de widget")
        }
    }
}

fn run_shell(command: &str) -> Result<()> {
    Command::new("zsh").arg("-c").arg(command).spawn()?;
    Ok(())
}
