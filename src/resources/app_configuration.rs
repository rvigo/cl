use crate::resources::utils::{from_toml, to_toml};
use anyhow::{bail, Context, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

const APP_HOME_DIR: &str = ".config/cl";
const COMMAND_FILE: &str = "commands.toml";
const APP_CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct AppConfiguration {
    config_home_path: Option<PathBuf>,
    command_file_path: Option<PathBuf>,
}

impl AppConfiguration {
    pub fn init() -> Result<AppConfiguration> {
        fn new(home_path: &Path) -> AppConfiguration {
            AppConfiguration {
                config_home_path: Some(home_path.to_path_buf()),
                command_file_path: Some(home_path.join(COMMAND_FILE)),
            }
        }

        fn load() -> Result<AppConfiguration> {
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
                //TODO should be moved to file_service.rs
                let data = match read_to_string(config_path) {
                    Ok(file) => file,
                    Err(error) => bail!("Cannot read {APP_CONFIG_FILE}: {error}"),
                };
                let config: AppConfiguration = from_toml(&data);
                validate_config(&config)?;

                Ok(config)
            } else {
                let new_config = create_new_files(&app_home_dir)?;
                Ok(new_config)
            }
        }

        fn create_new_files(app_home_dir: &Path) -> Result<AppConfiguration> {
            let new_config = new(app_home_dir);
            create_new_config(app_home_dir)?;
            create_empty_command_file(&app_home_dir.join(COMMAND_FILE))?;
            Ok(new_config)
        }

        fn validate_config(config: &AppConfiguration) -> Result<()> {
            if !config.command_file_path.as_ref().unwrap().exists() {
                create_empty_command_file(config.command_file_path.as_ref().unwrap())?;
            }

            Ok(())
        }

        fn create_new_config(home_path: &Path) -> Result<()> {
            let new_config = new(home_path);
            let config_as_str = to_toml(&new_config);
            let config_file_path = new_config
                .config_home_path
                .as_ref()
                .unwrap()
                .join(APP_CONFIG_FILE);

            save_file(config_as_str, config_file_path.as_path())
                .context("Something went wrong while saving the config file")?;

            Ok(())
        }

        fn create_empty_command_file(path: &Path) -> Result<()> {
            save_file(
                String::from(""), //empty toml file
                path,
            )
            .context("Something went wrong while saving the command file")
        }

        //TODO should be moved to file_service.rs
        fn save_file(toml_content_as_string: String, path: &Path) -> Result<()> {
            write(path, toml_content_as_string)?;
            Ok(())
        }

        load().context("Cannot load the config/command file")
    }

    pub fn command_file_path(&self) -> PathBuf {
        self.command_file_path
            .as_ref()
            .expect("command file should exist before call this method")
            .to_path_buf()
    }
}
