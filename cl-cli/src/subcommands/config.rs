use super::Subcommand;
use anyhow::{bail, Context, Result};
use cl_core::{config, Config as CoreConfig, LogLevel as ConfigLogLevel};
use clap::{Parser, Subcommand as ClapSubcommand, ValueEnum};
use dirs::home_dir;
use log::info;
use std::{
    env,
    fs::{write, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum LogLevel {
    Debug,
    Info,
    Error,
}

impl From<LogLevel> for ConfigLogLevel {
    fn from(value: LogLevel) -> ConfigLogLevel {
        match value {
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
        help = "Set the quiet mode"
    )]
    quiet_mode: Option<bool>,
    #[clap(
        value_parser,
        long,
        short = 'l',
        ignore_case = true,
        required = false,
        num_args(1),
        help = "Set the log level"
    )]
    log_level: Option<LogLevel>,
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
    fn run(&self, mut config: impl CoreConfig) -> Result<()> {
        if let Some(ConfigSubcommand::ZshWidget(_)) = &self.subcommand {
            return install_zsh_widget(config::get_config_path())
                .context("Failed to install zsh widget");
        }

        if let Some(quiet) = self.quiet_mode {
            config
                .change_and_save(|c| c.preferences_mut().set_quiet_mode(quiet))
                .if_ok(|| info!("quiet mode set to {quiet}"))
        } else if let Some(log_level) = self.log_level {
            config
                .change_and_save(|c| c.preferences_mut().set_log_level(log_level.into()))
                .if_ok(|| info!("log level set to {log_level:?}"))
        } else if let Some(highlight) = self.highlight_matches {
            config
                .change_and_save(|c| c.preferences_mut().set_highlight(highlight))
                .if_ok(|| info!("highlight matches set to {highlight}"))
        } else {
            println!("{}", config.printable());
            Ok(())
        }
    }
}

fn install_zsh_widget(app_home_dir: PathBuf) -> Result<()> {
    let shell = env::var("SHELL").unwrap_or_default();
    if !shell.contains("zsh") {
        bail!("Cannot install zsh widget on non-zsh shell! Actual $SHELL value is {shell}");
    }

    validate_fzf()?;

    const CL_WIDGET_NAME: &str = "cl-exec-widget";
    let widget = include_str!("../resources/zsh/cl-exec-widget");
    let dest_location = app_home_dir.join(CL_WIDGET_NAME);

    // creates new file
    write(&dest_location, widget)?;

    let home = home_dir().context("Cannot find users $HOME directory")?;
    let zshrc_file = home.join(".zshrc");
    let mut file = OpenOptions::new()
        .append(true)
        .open(&zshrc_file)
        .context(format!("Cannot open {} file", zshrc_file.display()))?;

    // append to the last line
    writeln!(file, "source {}", dest_location.display()).context(format!(
        "Cannot write to the .zshrc file. Please add `source {}` at the end of your .zshrc file",
        dest_location.display()
    ))?;

    info!("Done!!! Please restart your terminal and press <Ctrl+O> to access the widget");

    Ok(())
}

fn validate_fzf() -> Result<()> {
    let output = Command::new("zsh")
        .arg("-c")
        .args(["command", "-v", "fzf"])
        .output()
        .context("Cannot validate if fzf is installed")?;

    if !output.status.success() {
        bail!("This widget needs fzf to work. Please first install it and then reinstall de widget")
    }

    Ok(())
}

trait PrintableAppConfig {
    /// Represents a kind of pretty printable version of the AppConfig struct
    fn printable(&self) -> String;
}

impl<T> PrintableAppConfig for T
where
    T: CoreConfig,
{
    fn printable(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!("command-file: {:?}\n", self.command_file_path()));
        let preferences = self.preferences();
        result.push_str("preferences:\n");
        result.push_str(&format!("  quiet-mode: {}\n", preferences.quiet_mode()));
        result.push_str(&format!(
            "  log-level: {}\n",
            String::from(&preferences.log_level())
        ));
        result.push_str(&format!(
            "  highlight-matches: {}\n",
            preferences.highlight()
        ));

        result
    }
}

trait IfOk<T> {
    fn if_ok<F>(self, f: F) -> Result<T>
    where
        F: FnOnce(),
        Self: Sized;
}

impl<T> IfOk<T> for Result<T> {
    // If the `anyhow::Result` variant if `Ok(T)`, runs `f` and then returns `Ok(T)`
    fn if_ok<F>(self, f: F) -> Result<T>
    where
        F: FnOnce(),
        Self: Sized,
    {
        match self {
            Ok(ok) => {
                f();
                Ok(ok)
            }
            Err(err) => Err(err),
        }
    }
}

trait ConfigExt {
    fn change_and_save<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Self);
}

impl<T> ConfigExt for T
where
    T: CoreConfig,
{
    fn change_and_save<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Self),
    {
        f(self);
        self.save()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn should_find_the_widget_file() {
        let widget = include_str!("../resources/zsh/cl-exec-widget");

        assert!(!widget.is_empty())
    }
}
