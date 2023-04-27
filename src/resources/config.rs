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
    vi_keybindings_enabled: Option<bool>,
}

impl Options {
    pub fn new() -> Options {
        Self {
            default_quiet_mode: Some(false),
            log_level: Some(LogLevel::default()),
            highlight_matches: Some(true),
            vi_keybindings_enabled: Some(false),
        }
    }

    pub fn get_highlight(&mut self) -> bool {
        self.highlight_matches.unwrap_or(true)
    }

    pub fn set_highlight(&mut self, highlight: bool) {
        self.highlight_matches = Some(highlight);
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

    pub fn vi_enabled(&self) -> bool {
        self.vi_keybindings_enabled.unwrap_or(false)
    }

    pub fn enable_vi(&mut self, enable: bool) {
        self.vi_keybindings_enabled = Some(enable)
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
        // assuming that `Config::load` is the only public entrypoint, the options should never be `None`
        self.options.to_owned().unwrap()
    }

    pub fn get_command_file_path(&self) -> Result<PathBuf> {
        Ok(self
            .command_file_path
            .to_owned()
            .unwrap_or(self.get_app_home_dir().join(COMMAND_FILE)))
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

    pub fn get_log_level(&self) -> LogLevel {
        match self.options.as_ref().unwrap().get_log_level() {
            Ok(Some(log_level)) => log_level.to_owned(),
            _ => LogLevel::default(),
        }
    }

    pub fn set_log_level(&mut self, log_level: LogLevel) -> Result<()> {
        self.options.as_mut().unwrap().set_log_level(log_level);
        self.save()
    }

    pub fn get_default_quiet_mode(&self) -> bool {
        self.options
            .as_ref()
            .unwrap()
            .default_quiet_mode
            .unwrap_or(false)
    }

    pub fn set_default_quiet_mode(&mut self, quiet_mode: bool) -> Result<()> {
        self.options
            .as_mut()
            .unwrap()
            .set_default_quiet_mode(quiet_mode);
        self.save()
    }

    pub fn enable_vi_keybindings(&mut self, enable: bool) -> Result<()> {
        self.options.as_mut().unwrap().enable_vi(enable);
        self.save()
    }

    pub fn vi_keybindings_enabled(&self) -> bool {
        self.options.as_ref().unwrap().vi_enabled()
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
            if let Some(val) = &options.vi_keybindings_enabled {
                result.push_str(&format!("vi keybindings enabled: {val}\n"))
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
            should_save = true;
            self.app_home_dir = Some(PathBuf::from(APP_HOME_DIR))
        }
        if self.config_home_path.is_none() {
            should_save = true;
            self.config_home_path = Some(self.app_home_dir.as_ref().unwrap().join(APP_CONFIG_FILE));
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
        if self.options.as_ref().unwrap().highlight_matches.is_none() {
            should_save = true;
            self.options.as_mut().unwrap().highlight_matches = Some(true);
        }
        if self
            .options
            .as_ref()
            .unwrap()
            .vi_keybindings_enabled
            .is_none()
        {
            should_save = true;
            self.options.as_mut().unwrap().vi_keybindings_enabled = Some(false)
        }

        if let Ok(save) = self.ensure_command_file() {
            should_save = save
        }

        if should_save {
            self.save()
        } else {
            Ok(())
        }
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
                vi_keybindings_enabled: None,
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

    #[test]
    fn should_set_vi_keybindings() -> Result<()> {
        let mut config = builder();
        config.save()?;
        config.validate()?;

        assert_eq!(config.get_options().vi_enabled(), false);

        config.enable_vi_keybindings(true)?;

        assert_eq!(config.get_options().vi_enabled(), true);

        // get default value in case of None
        config.options.as_mut().unwrap().vi_keybindings_enabled = None;

        assert_eq!(config.get_options().vi_enabled(), false);

        tear_down(config)
    }
}
