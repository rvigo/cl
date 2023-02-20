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

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub enum LogLevel {
    Debug,
    #[default]
    Info,
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

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Options {
    default_quiet_mode: Option<bool>,
    log_level: Option<LogLevel>,
}

impl Options {
    pub fn new() -> Options {
        Self {
            default_quiet_mode: Some(false),
            log_level: Some(LogLevel::default()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Config {
    // temporally using the app_home_dir as an Option
    app_home_dir: Option<PathBuf>,
    config_home_path: Option<PathBuf>,
    command_file_path: Option<PathBuf>,
    options: Option<Options>,
}

impl Config {
    pub fn new() -> Result<Self> {
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

    pub fn get_log_level(&self) -> Result<Option<&LogLevel>> {
        Ok(self.options.as_ref().unwrap().log_level.as_ref())
    }

    // pub fn set_log_level(&mut self, log_level: LogLevel) -> Result<()> {
    //     self.log_level = Some(log_level);
    //     self.save()
    // }

    pub fn set_default_quiet_mode(&mut self, quiet_mode: bool) -> Result<()> {
        self.options.as_mut().unwrap().default_quiet_mode = Some(quiet_mode);
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
        let mut config_data = toml::to_string(self)?;
        config_data.push('\n');
        write(&config_file_path, config_data)?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let home = home_dir().expect("Could not find home directory");
        let config_file_path = home.join(APP_HOME_DIR).join(APP_CONFIG_FILE);
        if let Ok(config_data) = read_to_string(config_file_path) {
            if !config_data.is_empty() {
                let mut config: Self = toml::from_str(&config_data)?;
                config.validate()?;
                Ok(config)
            } else {
                Ok(Self::default())
            }
        } else {
            Self::new()
        }
    }

    pub fn validate(&mut self) -> Result<()> {
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
            }),
        };
        config.validate()?;

        Ok(config)
    }

    #[test]
    fn test_new() -> Result<()> {
        let config = builder()?;
        assert_eq!(config.get_log_level()?.unwrap(), &LogLevel::Info);
        assert_eq!(config.get_default_quiet_mode(), false);
        assert!(config.get_command_file_path()?.exists());
        assert!(config.get_config_file_path()?.exists());
        Ok(())
    }

    #[test]
    fn test_set_default_quiet_mode() -> Result<()> {
        let mut config = builder()?;
        config.set_default_quiet_mode(true)?;
        assert_eq!(config.get_default_quiet_mode(), true);
        Ok(())
    }
}
