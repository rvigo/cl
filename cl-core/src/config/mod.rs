pub mod default_config;

use crate::preferences::Preferences;
use anyhow::Result;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const CONFIG_ROOT_DIR: &str = ".config/cl";
pub const DEFAULT_CONFIG_FILE: &str = "config.toml";

#[derive(Clone, Serialize, Deserialize, Default)]
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

pub trait Config {
    fn load() -> Result<Self>
    where
        Self: Sized;

    fn save(&self) -> Result<()>;

    fn preferences(&self) -> &Preferences;

    fn preferences_mut(&mut self) -> &mut Preferences;

    fn command_file_path(&self) -> PathBuf;

    fn log_dir_path(&self) -> PathBuf;
}

pub fn get_config_path() -> PathBuf {
    let home = home_dir().expect("Cannot find home directory");
    home.join(CONFIG_ROOT_DIR).join(DEFAULT_CONFIG_FILE)
}
