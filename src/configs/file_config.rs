use crate::utils::{from_toml, to_toml};
use anyhow::{anyhow, Error, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

const HOMEDIR: &str = ".config/command_list";
const COMMAND_FILE: &str = "commands.toml";
const CONFIG: &str = "config.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn load_or_build_config() -> Result<FileConfig, Error> {
    match dirs::home_dir() {
        Some(home) => load_or_build(&home),
        None => Err(anyhow!("No $HOME directory found for config.")),
    }
}

fn load_or_build(path: &Path) -> Result<FileConfig, Error> {
    let home_path = Path::new(&path);

    let home_dir = home_path.join(HOMEDIR);
    if !home_dir.exists() {
        fs::create_dir_all(&home_dir)?;
    }

    let config_path = home_dir.join(CONFIG);
    let config = if config_path.exists() {
        let data = std::fs::read_to_string(config_path.clone())?;
        let mut loaded_config: FileConfig = from_toml(&data);

        if loaded_config.command_file_path.is_none() {
            loaded_config.command_file_path = Some(home_dir.join(COMMAND_FILE));
            save_config(&loaded_config, &config_path)?;
        }

        //ensure an empty toml file if commands.toml does not exists
        if loaded_config.command_file_path.is_some()
            && !loaded_config.command_file_path.as_ref().unwrap().exists()
        {
            save_file(
                String::from(""),
                loaded_config.command_file_path.as_ref().unwrap(),
            )?;
        }

        Ok(loaded_config)
    } else {
        let new_config = FileConfig::new(&home_dir);
        save_config(&new_config, &config_path)?;
        Ok(new_config)
    };

    config
}

fn save_config(config_to_save: &FileConfig, config_path: &Path) -> Result<()> {
    let s = to_toml(config_to_save);
    save_file(s, config_path)?;
    Ok(())
}

fn save_file(toml_content_as_string: String, path: &Path) -> Result<()> {
    match fs::write(path, toml_content_as_string) {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!(err)),
    }
}
