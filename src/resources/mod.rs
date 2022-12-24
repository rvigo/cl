pub(super) mod config;
pub(super) mod file_service;
pub mod log;

use anyhow::Result;

use crate::commands::Commands;

mod utils {
    pub fn to_toml<T>(value: &T) -> String
    where
        T: for<'de> serde::Deserialize<'de> + serde::Serialize,
    {
        toml::to_string(&value).expect("Unable to convert to toml")
    }
    pub fn from_toml<T>(text: &str) -> T
    where
        T: for<'de> serde::Deserialize<'de> + serde::Serialize,
    {
        toml::from_str(text).expect("Unable to parse file")
    }
}

pub fn load_commands() -> Result<Commands> {
    let command_items = file_service::load_commands_from_file()?;
    Ok(Commands::init(command_items))
}
