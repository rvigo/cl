use crate::utils::{from_toml, to_toml};
use anyhow::{bail, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

const COMMAND_LIST_HOME_DIR: &str = ".config/command_list";
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

    let home_dir = home.join(COMMAND_LIST_HOME_DIR);
    if !home_dir.exists() {
        create_dir_all(&home_dir)?;
        let new_config = FileConfig::new(&home_dir);
        create_new_config(&home_dir)?;
        return Ok(new_config);
    }

    let config_path = home_dir.join(CONFIG_FILE);
    let config = if config_path.exists() {
        let mut config: FileConfig = from_toml(&read_to_string(config_path.clone())?);

        validate_config(&mut config, home_dir)?;

        Ok(config)
    } else {
        let new_config = FileConfig::new(&home_dir);
        create_new_config(&home_dir)?;
        Ok(new_config)
    };

    config
}

fn validate_config(config: &mut FileConfig, home_dir: PathBuf) -> Result<()> {
    if config.command_file_path.is_none() {
        config.command_file_path = Some(home_dir.join(COMMAND_FILE));
        create_new_config(&home_dir)?;
    }

    if config.command_file_path.is_some() && !config.command_file_path.as_ref().unwrap().exists() {
        create_empty_command_file(config.command_file_path.as_ref().unwrap())?;
    }

    Ok(())
}

fn create_new_config(home_dir: &PathBuf) -> Result<()> {
    let new_config = FileConfig::new(&home_dir);
    let config_as_str = to_toml(&new_config);
    save_file(
        config_as_str,
        new_config.config_home_path.as_ref().unwrap().as_path(),
    )?;
    Ok(())
}

fn create_empty_command_file(path: &PathBuf) -> Result<()> {
    save_file(
        String::from(""), //empty toml file
        &path,
    )
}

fn save_file(toml_content_as_string: String, path: &Path) -> Result<()> {
    match write(path, toml_content_as_string) {
        Ok(_) => Ok(()),
        Err(err) => bail!(err),
    }
}
