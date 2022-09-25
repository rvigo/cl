use super::file_service;
use crate::resources::utils::{from_toml, to_toml};
use anyhow::{Context, Result};
use dirs::home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::create_dir_all,
    path::{Path, PathBuf},
};
use toml::Value;

const APP_HOME_DIR: &str = ".config/cl";
const COMMAND_FILE: &str = "commands.toml";
const APP_CONFIG_FILE: &str = "config.toml";

lazy_static! {
    //make a Config "instance" globally available
    pub static ref CONFIG: Config = load()
        .context("Cannot load the config/command file")
        .unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    config_home_path: PathBuf,
    command_file_path: PathBuf,
    default_quiet_mode: bool,
}

impl Config {
    pub fn get_command_file_path(&self) -> PathBuf {
        self.command_file_path.to_path_buf()
    }
    pub fn quiet_mode(&self) -> bool {
        self.default_quiet_mode
    }
}

pub fn load() -> Result<Config> {
    let home_dir = &home_dir().context("no $HOME????")?;
    let home = Path::new(home_dir);

    let app_home_dir = home.join(APP_HOME_DIR);
    if !app_home_dir.exists() {
        create_dir_all(&app_home_dir)?;
        let new_config = create_new_files(&app_home_dir)?;
        return Ok(new_config);
    }

    let config_path = app_home_dir.join(APP_CONFIG_FILE);
    if config_path.exists() {
        let mut should_save_default_quiet_mode_flag = false;
        let data = file_service::open_file(&config_path)?;

        // ensures new parameter in config.toml
        // should be removed in future versions
        let mut config_map: HashMap<String, Value> = from_toml(&data);
        if !config_map.contains_key("default_quiet_mode") {
            config_map.insert(String::from("default_quiet_mode"), Value::Boolean(false));
            should_save_default_quiet_mode_flag = true;
        }

        let config: Config = from_toml(&to_toml(&config_map));
        validate_config(&config)?;

        if should_save_default_quiet_mode_flag {
            update_config(&config)?;
        }

        Ok(config)
    } else {
        let new_config = create_new_files(&app_home_dir)?;
        Ok(new_config)
    }
}

pub fn set_quiet_mode(quiet_mode_flag: bool) -> Result<()> {
    println!("Info: setting default_quiet_mode to {}", quiet_mode_flag);
    let mut config: Config = load()?;

    if config.default_quiet_mode != quiet_mode_flag {
        config.default_quiet_mode = quiet_mode_flag;
    }

    update_config(&config)?;
    Ok(())
}

fn new(home_path: &Path) -> Config {
    Config {
        config_home_path: home_path.to_path_buf(),
        command_file_path: home_path.join(COMMAND_FILE),
        default_quiet_mode: false,
    }
}

fn create_new_files(app_home_dir: &Path) -> Result<Config> {
    let new_config = new(app_home_dir);
    create_new_config(app_home_dir)?;
    create_empty_command_file(&app_home_dir.join(COMMAND_FILE))?;
    Ok(new_config)
}

fn validate_config(config: &Config) -> Result<()> {
    if !config.command_file_path.exists() {
        create_empty_command_file(&config.command_file_path)?;
    }

    Ok(())
}

fn create_new_config(home_path: &Path) -> Result<()> {
    let new_config = new(home_path);
    let config_as_str = to_toml(&new_config);
    let config_file_path = new_config.config_home_path.join(APP_CONFIG_FILE);
    file_service::save_file(config_as_str, config_file_path.as_path())
        .context("Something went wrong while creating the config file")?;

    Ok(())
}

fn update_config(config: &Config) -> Result<()> {
    let config_as_str = to_toml(config);
    file_service::save_file(config_as_str, config.config_home_path.as_path())
        .context("Something went wrong while updating the config file")?;

    Ok(())
}

fn create_empty_command_file(path: &Path) -> Result<()> {
    file_service::save_file(
        String::from(""), //empty toml file
        path,
    )
    .context("Something went wrong while creating a command file")
}
