use super::Subcommand;
use anyhow::{bail, Context, Result};
use cl_core::{config, Config as CoreConfig, LogLevel as ConfigLogLevel};
use clap::{Parser, Subcommand as ClapSubcommand, ValueEnum};
use dirs::home_dir;
use std::{
    env,
    fs::{write, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
};
use tracing::{debug, info};

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
            return install_zsh_widget(config::get_config_path()?)
                .context("Failed to install zsh widget");
        }

        let mut any_flag = false;

        if let Some(quiet) = self.quiet_mode {
            any_flag = true;
            config
                .change_and_save(|c| c.preferences_mut().set_quiet_mode(quiet))
                .if_ok(|| info!(target: "cl::config", quiet_mode = quiet, "quiet mode updated"))?;
        }

        if let Some(log_level) = self.log_level {
            any_flag = true;
            config
                .change_and_save(|c| c.preferences_mut().set_log_level(log_level.into()))
                .if_ok(
                    || info!(target: "cl::config", log_level = ?log_level, "log level updated"),
                )?;
        }

        if let Some(highlight) = self.highlight_matches {
            any_flag = true;
            config
                .change_and_save(|c| c.preferences_mut().set_highlight(highlight))
                .if_ok(|| info!(target: "cl::config", highlight_matches = highlight, "highlight matches updated"))?;
        }

        if !any_flag {
            println!("{}", printable(&config));
        }

        Ok(())
    }
}

fn install_zsh_widget(app_home_dir: PathBuf) -> Result<()> {
    let shell = match env::var("SHELL") {
        Ok(s) => s,
        Err(_) => bail!("$SHELL environment variable is not set; cannot determine your shell"),
    };
    if !shell.contains("zsh") {
        bail!("Cannot install zsh widget on non-zsh shell! Actual $SHELL value is {shell}");
    }

    validate_fzf()?;

    const CL_WIDGET_NAME: &str = "cl-exec-widget";
    let widget = include_str!("../resources/zsh/cl-exec-widget");
    let dest_location = app_home_dir.join(CL_WIDGET_NAME);

    // creates new file
    write(&dest_location, widget)?;
    debug!(target: "cl::config", path = %dest_location.display(), "widget file written");

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
    debug!(target: "cl::config", file = %zshrc_file.display(), "source line appended to zshrc");

    info!(target: "cl::config", "zsh widget installed; restart your terminal and press <Ctrl+O> to access it");

    Ok(())
}

fn validate_fzf() -> Result<()> {
    let output = Command::new("zsh")
        .arg("-c")
        .args(["command", "-v", "fzf"])
        .output()
        .context("Cannot validate if fzf is installed")?;

    if !output.status.success() {
        bail!(
            "This widget needs fzf to work. Please first install it and then reinstall the widget"
        )
    }

    Ok(())
}

fn printable(config: &impl CoreConfig) -> String {
    let mut result = String::new();
    result.push_str(&format!("command-file: {:?}\n", config.command_file_path()));
    let preferences = config.preferences();
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
    use super::*;
    use cl_core::{LogLevel as CoreLogLevel, Preferences};
    use std::path::PathBuf;

    #[test]
    fn should_find_the_widget_file() {
        let widget = include_str!("../resources/zsh/cl-exec-widget");

        assert!(!widget.is_empty())
    }

    #[test]
    fn if_ok_runs_closure_on_ok() {
        let mut ran = false;
        let result: anyhow::Result<i32> = Ok(42);
        let out = result.if_ok(|| ran = true);
        assert!(out.is_ok());
        assert_eq!(out.unwrap(), 42);
        assert!(ran);
    }

    #[test]
    fn if_ok_does_not_run_closure_on_err() {
        let mut ran = false;
        let result: anyhow::Result<i32> = Err(anyhow::anyhow!("oops"));
        let out = result.if_ok(|| ran = true);
        assert!(out.is_err());
        assert!(!ran);
    }

    struct MockConfig {
        preferences: Preferences,
        command_file: PathBuf,
    }

    impl CoreConfig for MockConfig {
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
            self.command_file.clone()
        }
        fn log_dir_path(&self) -> anyhow::Result<PathBuf> {
            Ok(PathBuf::from("/tmp"))
        }
    }

    #[test]
    fn printable_contains_expected_keys() {
        let config = MockConfig {
            preferences: Preferences::default(),
            command_file: PathBuf::from("/tmp/commands.toml"),
        };
        let output = printable(&config);
        assert!(output.contains("command-file:"));
        assert!(output.contains("quiet-mode:"));
        assert!(output.contains("log-level:"));
        assert!(output.contains("highlight-matches:"));
    }

    #[test]
    fn printable_reflects_non_default_preferences() {
        let mut prefs = Preferences::default();
        prefs.set_quiet_mode(true);
        prefs.set_log_level(CoreLogLevel::Debug);
        let config = MockConfig {
            preferences: prefs,
            command_file: PathBuf::from("/tmp/commands.toml"),
        };
        let output = printable(&config);
        assert!(output.contains("quiet-mode: true"));
        assert!(output.contains("log-level: debug"));
    }
}
