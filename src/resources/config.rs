use super::file_service;
use crate::resources::utils::{from_toml, to_toml};
use anyhow::{Context, Result};
use dirs::home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
};

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
}

impl Config {
    pub fn get_command_file_path(&self) -> PathBuf {
        self.command_file_path.to_path_buf()
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
        let data = file_service::open_file(&config_path)?;
        let config: Config = from_toml(&data);
        validate_config(&config)?;

        Ok(config)
    } else {
        let new_config = create_new_files(&app_home_dir)?;
        Ok(new_config)
    }
}

fn new(home_path: &Path) -> Config {
    Config {
        config_home_path: home_path.to_path_buf(),
        command_file_path: home_path.join(COMMAND_FILE),
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

fn create_empty_command_file(path: &Path) -> Result<()> {
    file_service::save_file(
        String::from(""), //empty toml file
        path,
    )
    .context("Something went wrong while creating a command file")
}
