use crate::utils::{from_toml, to_toml};
use anyhow::{bail, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

const APP_HOME_DIR: &str = ".config/cl";
const COMMAND_FILE: &str = "commands.toml";
const CONFIG_FILE: &str = "config.toml";

#[derive(Clone, Serialize, Deserialize)]
pub struct FileConfig {
    pub config_home_path: Option<PathBuf>,
    pub command_file_path: Option<PathBuf>,
}

impl FileConfig {
    pub fn new(home_path: &Path) -> Self {
        Self {
            config_home_path: Some(home_path.to_path_buf()),
            command_file_path: Some(home_path.join(COMMAND_FILE)),
        }
    }
}

pub fn load() -> Result<FileConfig> {
    let home_dir = &home_dir().expect("no $HOME????");
    let home = Path::new(home_dir);

    let app_home_dir = home.join(APP_HOME_DIR);
    if !app_home_dir.exists() {
        create_dir_all(&app_home_dir)?;
        let new_config = create_new_files(&app_home_dir)?;
        return Ok(new_config);
    }

    let config_path = app_home_dir.join(CONFIG_FILE);
    if config_path.exists() {
        let mut config: FileConfig = from_toml(&read_to_string(config_path)?);
        validate_config(&mut config)?;

        Ok(config)
    } else {
        let new_config = create_new_files(&app_home_dir)?;
        Ok(new_config)
    }
}

fn create_new_files(app_home_dir: &Path) -> Result<FileConfig> {
    let new_config = FileConfig::new(app_home_dir);
    create_new_config(app_home_dir)?;
    create_empty_command_file(&app_home_dir.join(COMMAND_FILE))?;
    Ok(new_config)
}

fn validate_config(config: &mut FileConfig) -> Result<()> {
    if !config.command_file_path.as_ref().unwrap().exists() {
        create_empty_command_file(config.command_file_path.as_ref().unwrap())?;
    }

    Ok(())
}

fn create_new_config(home_path: &Path) -> Result<()> {
    let new_config = FileConfig::new(home_path);
    let config_as_str = to_toml(&new_config);
    let config_file_path = new_config
        .config_home_path
        .as_ref()
        .unwrap()
        .join(CONFIG_FILE);

    match save_file(config_as_str, config_file_path.as_path()) {
        Ok(_) => Ok(()),
        Err(error) => bail!("something went wrong: {error}"),
    }
}

fn create_empty_command_file(path: &Path) -> Result<()> {
    save_file(
        String::from(""), //empty toml file
        path,
    )
}

fn save_file(toml_content_as_string: String, path: &Path) -> Result<()> {
    match write(path, toml_content_as_string) {
        Ok(_) => Ok(()),
        Err(err) => bail!(
            "something wrong while saving config at {:?}: {}",
            path.to_str(),
            err
        ),
    }
}
