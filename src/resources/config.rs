use anyhow::Result;
use dirs::home_dir;
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::PathBuf,
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
    highlitght_matches: Option<bool>,
}

impl Options {
    pub fn new() -> Options {
        Self {
            default_quiet_mode: Some(false),
            log_level: Some(LogLevel::default()),
            highlitght_matches: Some(true),
        }
    }

    pub fn get_highlight(&mut self) -> Result<Option<bool>> {
        Ok(self.highlitght_matches)
    }

    pub fn set_highlight(&mut self, highlight: bool) {
        self.highlitght_matches = Some(highlight);
    }

    pub fn get_log_level(&self) -> Result<Option<&LogLevel>> {
        Ok(self.log_level.as_ref())
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

#[derive(Serialize, Deserialize, Default, PartialEq)]
pub struct Config {
    /// the location of the config file. Defaults to `$HOME/.config/cl`
    app_home_dir: Option<PathBuf>,
    config_home_path: Option<PathBuf>,
    command_file_path: Option<PathBuf>,
    options: Option<Options>,
}

impl Config {
    pub fn get_options(&self) -> Options {
        // assuming that `Config::load` is the only public entrypoint, the options should never be `None`
        self.options.to_owned().unwrap()
    }

    pub fn get_command_file_path(&self) -> Result<PathBuf> {
        Ok(self.get_app_home_dir().join(COMMAND_FILE))
    }

    pub fn get_app_home_dir(&self) -> PathBuf {
        self.app_home_dir.as_ref().unwrap().to_owned()
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

    pub fn get_log_level(&self) -> Result<Option<&LogLevel>> {
        self.options.as_ref().unwrap().get_log_level()
    }

    pub fn set_log_level(&mut self, log_level: LogLevel) -> Result<()> {
        self.options.as_mut().unwrap().set_log_level(log_level);
        self.save()
    }

    pub fn set_default_quiet_mode(&mut self, quiet_mode: bool) -> Result<()> {
        self.options
            .as_mut()
            .unwrap()
            .set_default_quiet_mode(quiet_mode);
        self.save()
    }

    pub fn get_default_quiet_mode(&self) -> bool {
        self.options
            .as_ref()
            .unwrap()
            .default_quiet_mode
            .unwrap_or(false)
    }

    pub fn save(&self) -> Result<()> {
        let app_home_dir = self.get_app_home_dir();
        if !app_home_dir.exists() {
            debug!("creating dirs: {app_home_dir:?}");
            create_dir_all(&app_home_dir)?
        }
        let config_file_path = app_home_dir.join(self.get_config_file_path()?);
        debug!("saving file to: {config_file_path:?}");
        let config_data = toml::to_string(self)?;
        write(&config_file_path, config_data)?;
        Ok(())
    }

    /// Loads the config file. Name defaults to `config.toml`
    ///
    /// Creates a new one if the config data is empty or is missing
    pub fn load() -> Result<Self> {
        let home = home_dir().expect("Could not find home directory");
        let config_file_path = home.join(APP_HOME_DIR).join(APP_CONFIG_FILE);
        if let Ok(config_data) = read_to_string(config_file_path) {
            if !config_data.is_empty() {
                let mut config: Self = toml::from_str(&config_data)?;
                config.validate()?;
                Ok(config)
            } else {
                Self::new()
            }
        } else {
            Self::new()
        }
    }

    fn new() -> Result<Self> {
        let home_dir = home_dir().expect("Could not find home directory");

        let mut config = Self {
            app_home_dir: Some(home_dir.join(APP_HOME_DIR)),
            config_home_path: None,
            command_file_path: None,
            options: Some(Options::new()),
        };

        config.validate()?;
        config.save()?;

        Ok(config)
    }

    fn get_config_file_path(&self) -> Result<PathBuf> {
        Ok(self.get_app_home_dir().join(APP_CONFIG_FILE))
    }

    /// Validates the config properties
    ///
    /// If some property is missing, ensures a default value and then saves the file
    fn validate(&mut self) -> Result<()> {
        let mut should_save = false;

        if self.app_home_dir.is_none() {
            should_save = true;
            self.app_home_dir = Some(PathBuf::from(APP_HOME_DIR))
        }
        if self.config_home_path.is_none() {
            should_save = true;
            self.config_home_path = Some(self.app_home_dir.as_ref().unwrap().join(APP_CONFIG_FILE));
        }
        if self.command_file_path.is_none() {
            should_save = true;
            self.command_file_path = Some(self.app_home_dir.as_ref().unwrap().join(COMMAND_FILE));
        }
        if self.options.is_none() {
            should_save = true;
            self.options = Some(Options::new())
        }
        if self.options.as_ref().unwrap().default_quiet_mode.is_none() {
            should_save = true;
            self.options.as_mut().unwrap().default_quiet_mode = Some(false);
        }
        if self.options.as_ref().unwrap().log_level.is_none() {
            should_save = true;
            self.options.as_mut().unwrap().log_level = Some(LogLevel::default());
        }
        if self.options.as_ref().unwrap().highlitght_matches.is_none() {
            should_save = true;
            self.options.as_mut().unwrap().highlitght_matches = Some(true);
        }

        if should_save {
            self.save()
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env::temp_dir;

    fn builder() -> Result<Config> {
        let mut config = Config {
            app_home_dir: Some(temp_dir().to_path_buf()),
            config_home_path: None,
            command_file_path: None,
            options: Some(Options {
                default_quiet_mode: None,
                log_level: None,
                highlitght_matches: None,
            }),
        };
        config.validate()?;

        Ok(config)
    }

    #[test]
    fn should_create_a_new_config() -> Result<()> {
        let config = builder()?;
        assert_eq!(config.get_log_level()?.unwrap(), &LogLevel::Error);
        assert_eq!(config.get_default_quiet_mode(), false);
        assert!(config.get_command_file_path()?.exists());
        assert!(config.get_config_file_path()?.exists());
        Ok(())
    }

    #[test]
    fn should_set_default_quiet_mode() -> Result<()> {
        let mut config = builder()?;

        assert_eq!(config.get_default_quiet_mode(), false);

        config.set_default_quiet_mode(true)?;

        assert_eq!(config.get_default_quiet_mode(), true);

        Ok(())
    }

    #[test]
    fn should_set_log_level() -> Result<()> {
        let mut config = builder()?;

        assert_eq!(config.get_log_level().unwrap(), Some(&LogLevel::Error));

        config.set_log_level(LogLevel::Debug)?;

        assert_eq!(config.get_log_level().unwrap(), Some(&LogLevel::Debug));

        Ok(())
    }
}
