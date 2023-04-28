use anyhow::{Context, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

const APP_HOME_DIR: &str = ".config/cl";
const COMMAND_FILE: &str = "commands.toml";
const APP_CONFIG_FILE: &str = "config.toml";

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    #[default]
    Error,
}

impl From<&LogLevel> for String {
    fn from(log_level: &LogLevel) -> Self {
        match log_level {
            LogLevel::Debug => String::from("debug"),
            LogLevel::Info => String::from("info"),
            LogLevel::Error => String::from("error"),
        }
    }
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "debug " => LogLevel::Debug,
            "info" => LogLevel::Info,
            "error" => LogLevel::Error,
            _ => LogLevel::default(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Options {
    default_quiet_mode: Option<bool>,
    log_level: Option<LogLevel>,
    highlight_matches: Option<bool>,
}

impl Options {
    pub fn new() -> Options {
        Self {
            default_quiet_mode: Some(false),
            log_level: Some(LogLevel::default()),
            highlight_matches: Some(true),
        }
    }

    pub fn get_highlight(&mut self) -> bool {
        self.highlight_matches.unwrap_or(true)
    }

    pub fn set_highlight(&mut self, highlight: bool) {
        self.highlight_matches = Some(highlight);
    }

    pub fn get_log_level(&self) -> LogLevel {
        self.log_level
            .as_ref()
            .unwrap_or(&LogLevel::default())
            .to_owned()
    }

    pub fn set_log_level(&mut self, log_level: LogLevel) {
        self.log_level = Some(log_level);
    }

    pub fn get_default_quiet_mode(&self) -> bool {
        self.default_quiet_mode.unwrap_or(false)
    }

    pub fn set_default_quiet_mode(&mut self, quiet_mode: bool) {
        self.default_quiet_mode = Some(quiet_mode);
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct Config {
    /// the location of the config file. Defaults to `$HOME/.config/cl`
    app_home_dir: Option<PathBuf>,
    config_home_path: Option<PathBuf>,
    command_file_path: Option<PathBuf>,
    options: Option<Options>,
}

impl Config {
    pub fn get_options(&self) -> Options {
        // assuming that `Config::load` is the only public entrypoint, this method should never return a default `Options`
        self.options
            .as_ref()
            .map_or_else(Options::default, |options| options.to_owned())
    }

    pub fn get_command_file_path(&self) -> PathBuf {
        self.command_file_path.as_ref().map_or_else(
            || self.get_app_home_dir().join(COMMAND_FILE),
            |p| p.to_owned(),
        )
    }

    /// Should return a default value if not present?
    pub fn get_app_home_dir(&self) -> PathBuf {
        self.app_home_dir
            .as_ref()
            .unwrap_or(
                &home_dir()
                    .expect("Cannot evaluate $HOME")
                    .join(APP_HOME_DIR),
            )
            .to_owned()
    }

    // pub fn set_config_home_path<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
    //     self.config_home_path = Some(path.as_ref().to_path_buf());
    //     self.save()
    // }

    // pub fn set_command_file_path<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
    //     self.command_file_path = Some(path.as_ref().to_path_buf());
    //     self.save()
    // }

    pub fn set_highlight(&mut self, highlight: bool) -> Result<()> {
        self.options.as_mut().unwrap().set_highlight(highlight);
        self.save()
    }

    pub fn get_log_level(&self) -> LogLevel {
        self.options
            .as_ref()
            .map_or_else(LogLevel::default, |options| options.get_log_level())
    }

    pub fn set_log_level(&mut self, log_level: LogLevel) -> Result<()> {
        self.options.as_mut().unwrap().set_log_level(log_level);
        self.save()
    }

    pub fn get_default_quiet_mode(&self) -> bool {
        self.options
            .as_ref()
            .map_or_else(|| false, |options| options.get_default_quiet_mode())
    }

    pub fn set_default_quiet_mode(&mut self, quiet_mode: bool) -> Result<()> {
        self.options
            .as_mut()
            .unwrap()
            .set_default_quiet_mode(quiet_mode);
        self.save()
    }

    pub fn save(&self) -> Result<()> {
        let app_home_dir = self.get_app_home_dir();
        if !app_home_dir.exists() {
            create_dir_all(&app_home_dir).context(format!("Cannot create {app_home_dir:?}"))?
        }
        let config_file_path = app_home_dir.join(self.get_config_file_path()?);
        let config_data = toml::to_string(self)?;
        write(config_file_path, config_data)?;
        Ok(())
    }

    /// Loads the config file. Name defaults to `config.toml`
    ///
    /// Creates a new one if the config data is empty or is missing
    pub fn load() -> Result<Self> {
        let home = home_dir().context("Could not find home directory")?;
        let config_file_path = home.join(APP_HOME_DIR).join(APP_CONFIG_FILE);
        if let Ok(config_data) = read_to_string(config_file_path) {
            if !config_data.is_empty() {
                let mut config: Self = toml::from_str(&config_data)?;
                config
                    .validate()
                    .context("Cannot validate the loaded config")?;
                return Ok(config);
            }
        }
        Self::new()
    }

    fn new() -> Result<Self> {
        let home_dir = home_dir().context("Could not find home directory")?;
        let mut config = Self {
            app_home_dir: Some(home_dir.join(APP_HOME_DIR)),
            config_home_path: None,
            command_file_path: None,
            options: Some(Options::new()),
        };
        config.save().context("Cannot save the config file")?;
        config
            .validate()
            .context("Cannot validate the new config")?;

        Ok(config)
    }

    fn create_empty_command_file<P: AsRef<Path>>(&self, command_file_path: P) -> Result<()> {
        write(command_file_path, "")?;
        Ok(())
    }

    fn get_config_file_path(&self) -> Result<PathBuf> {
        Ok(self.get_app_home_dir().join(APP_CONFIG_FILE))
    }

    pub fn printable_string(&self) -> String {
        let mut result = String::new();
        if let Some(path) = &self.app_home_dir {
            result.push_str(&format!("app home: {path:?}\n"));
        }
        if let Some(path) = &self.config_home_path {
            result.push_str(&format!("config home: {path:?}\n"));
        }
        if let Some(path) = &self.command_file_path {
            result.push_str(&format!("command file location: {path:?}\n"));
        }
        if let Some(options) = &self.options {
            if let Some(val) = &options.default_quiet_mode {
                result.push_str(&format!("quiet mode: {val}\n"));
            }
            if let Some(val) = &options.log_level {
                result.push_str(&format!("log level: {val:?}\n"));
            }
            if let Some(val) = &options.highlight_matches {
                result.push_str(&format!("highlight matches: {val}\n"));
            }
        }
        result
    }

    /// Validates the config properties
    ///
    /// If some property is missing, ensures a default value and then saves the file
    fn validate(&mut self) -> Result<()> {
        let mut should_save = false;

        if self.app_home_dir.is_none() {
            self.app_home_dir = Some(PathBuf::from(APP_HOME_DIR));
            should_save |= true;
        }
        if self.config_home_path.is_none() {
            self.config_home_path = self
                .app_home_dir
                .as_ref()
                .map(|dir| dir.join(APP_CONFIG_FILE));
            should_save |= true;
        }

        if self.options.is_some() {
            should_save |= self.validate_options();
        } else {
            self.options = Some(Options::default());
            should_save |= self.validate_options()
        }

        should_save |= self.ensure_command_file().unwrap_or(false);

        if should_save {
            self.save()
        } else {
            Ok(())
        }
    }

    fn validate_options(&mut self) -> bool {
        let mut should_save = false;
        if let Some(options) = self.options.as_mut() {
            if options.default_quiet_mode.is_none() {
                options.default_quiet_mode = Some(false);
                should_save |= true;
            }
            if options.log_level.is_none() {
                options.log_level = Some(LogLevel::default());
                should_save |= true;
            }
            if options.highlight_matches.is_none() {
                options.highlight_matches = Some(true);
                should_save |= true;
            }
        }
        should_save
    }

    fn ensure_command_file(&mut self) -> Result<bool> {
        let mut has_changes = false;
        if let Some(command_file) = &self.command_file_path {
            if !command_file.exists() {
                self.create_empty_command_file(command_file)
                    .context(format!(
                        "Cannot create an empty commands file at {command_file:?}"
                    ))?;
            }
            Ok(has_changes)
        } else {
            let path = self.app_home_dir.as_ref().unwrap().join(COMMAND_FILE);
            self.command_file_path = Some(path.clone());
            self.create_empty_command_file(&path)
                .context(format!("Cannot create an empty commands file at {path:?}"))?;
            has_changes = true;
            Ok(has_changes)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        env::temp_dir,
        fs,
        sync::atomic::{AtomicUsize, Ordering},
    };

    fn get_id() -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    fn builder() -> Config {
        let tmp = temp_dir();
        let config_dir = tmp.join(format!(".config{}", get_id()));
        Config {
            app_home_dir: Some(config_dir),
            config_home_path: None,
            command_file_path: None,
            options: Some(Options {
                default_quiet_mode: None,
                log_level: None,
                highlight_matches: None,
            }),
        }
    }

    fn tear_down(config: Config) -> Result<()> {
        if config.command_file_path.as_ref().is_some()
            && config.command_file_path.as_ref().unwrap().exists()
        {
            fs::remove_file(config.command_file_path.as_ref().unwrap())?;
        }
        if config.config_home_path.as_ref().is_some()
            && config.config_home_path.as_ref().unwrap().exists()
        {
            fs::remove_file(config.config_home_path.as_ref().unwrap())?;
        }
        fs::remove_dir(config.get_app_home_dir()).unwrap();

        Ok(())
    }

    #[test]
    fn should_save_a_new_config() -> Result<()> {
        let mut config = builder();

        assert!(config.app_home_dir.is_some());
        assert!(config.app_home_dir.as_ref().unwrap().try_exists().is_ok());
        assert_eq!(
            config.app_home_dir.as_ref().unwrap().try_exists().unwrap(),
            false
        );

        config.save()?;
        config.validate()?;

        assert!(config.app_home_dir.is_some());
        assert!(config.app_home_dir.as_ref().unwrap().try_exists().is_ok());
        assert_eq!(
            config.app_home_dir.as_ref().unwrap().try_exists().unwrap(),
            true
        );

        tear_down(config)
    }

    #[test]
    fn should_create_a_new_commands_file() -> Result<()> {
        let mut config = builder();
        config.save()?;
        config.validate()?;

        assert!(config.command_file_path.as_ref().is_some());
        assert!(config.command_file_path.as_ref().unwrap().exists());

        tear_down(config)
    }

    #[test]
    fn should_set_default_quiet_mode() -> Result<()> {
        let mut config = builder();
        config.save()?;
        config.validate()?;

        assert_eq!(config.get_default_quiet_mode(), false);

        config.set_default_quiet_mode(true)?;

        assert_eq!(config.get_default_quiet_mode(), true);

        tear_down(config)
    }

    #[test]
    fn should_set_log_level() -> Result<()> {
        let mut config = builder();
        config.save()?;
        config.validate()?;

        assert_eq!(config.get_log_level(), LogLevel::Error);

        config.set_log_level(LogLevel::Debug)?;

        assert_eq!(config.get_log_level(), LogLevel::Debug);

        tear_down(config)
    }

    #[test]
    fn should_set_highlight() -> Result<()> {
        let mut config = builder();
        config.save()?;
        config.validate()?;

        assert_eq!(config.get_options().get_highlight(), true);

        config.set_highlight(false)?;

        assert_eq!(config.get_options().get_highlight(), false);

        // get default value in case of None
        config.options.as_mut().unwrap().highlight_matches = None;

        assert_eq!(config.get_options().get_highlight(), true);

        tear_down(config)
    }
}
