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

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    config_home_path: Option<PathBuf>,
    command_file_path: Option<PathBuf>,
    default_quiet_mode: Option<bool>,
    app_home_dir: PathBuf,
}

impl Config {
    pub fn get_command_file_path(&self) -> PathBuf {
        self.command_file_path
            .as_ref()
            .unwrap_or(&self.app_home_dir.join(COMMAND_FILE))
            .to_path_buf()
    }

    pub fn quiet_mode(&self) -> bool {
        self.default_quiet_mode.unwrap_or(false)
    }

    pub fn get_app_home_dir(&self) -> PathBuf {
        self.app_home_dir.clone()
    }
}

fn get_user_home_dir() -> Result<PathBuf> {
    let home_dir = &home_dir().context("no $HOME????")?;
    Ok(Path::new(home_dir).to_path_buf())
}

pub fn load() -> Result<Config> {
    let home = get_user_home_dir()?;
    let app_home_dir = home.join(APP_HOME_DIR);

    //if app_home_dir (default: ~/.config/cl) doesnt exists, create the dir and the files
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
        create_new_files(&app_home_dir)
    }
}

pub fn set_quiet_mode(quiet_mode_flag: bool) -> Result<()> {
    println!("Info: Setting default_quiet_mode to {quiet_mode_flag}");
    let mut config: Config = load()?;

    if let Some(default_quiet_mode) = config.default_quiet_mode {
        if default_quiet_mode != quiet_mode_flag {
            config.default_quiet_mode = Some(quiet_mode_flag);
        }
    }

    update_config(&config)?;
    Ok(())
}

fn new(home_path: &Path) -> Config {
    let app_home_dir = home_path.to_path_buf();
    let config_home_path = Some(home_path.join(APP_CONFIG_FILE));
    let command_file_path = Some(home_path.join(COMMAND_FILE));
    let default_quiet_mode = Some(false);

    Config {
        app_home_dir,
        config_home_path,
        command_file_path,
        default_quiet_mode,
    }
}

fn create_new_files(app_home_dir: &Path) -> Result<Config> {
    create_empty_command_file(&app_home_dir.join(COMMAND_FILE))?;
    create_new_config(app_home_dir)
}

fn validate_config(config: &Config) -> Result<()> {
    if let Some(command_file_path) = &config.command_file_path {
        if !command_file_path.clone().exists() {
            create_empty_command_file(command_file_path)?;
        }
    }

    Ok(())
}

fn create_new_config(app_home_dir: &Path) -> Result<Config> {
    let new_config = new(app_home_dir);
    let config_as_str = to_toml(&new_config);
    let config_file_path = app_home_dir.join(APP_CONFIG_FILE);

    file_service::save_file(config_as_str, config_file_path.as_path())
        .context("Something went wrong while creating the config file")?;

    Ok(new_config)
}

fn update_config(config: &Config) -> Result<()> {
    let config_as_str = to_toml(config);
    file_service::save_file(config_as_str, config.config_home_path.as_ref().unwrap())
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
