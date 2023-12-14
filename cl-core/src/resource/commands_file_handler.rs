use super::{fs_wrapper::macros::write, toml::TomlFileHandler};
use crate::{hashmap, resource::errors::FileError, CommandMap};
use anyhow::Result;
use log::debug;
use std::path::{Path, PathBuf};

pub struct CommandsFileHandler {
    command_file_path: PathBuf,
    toml: TomlFileHandler,
}

impl CommandsFileHandler {
    pub fn new(command_file_path: PathBuf) -> CommandsFileHandler {
        Self {
            command_file_path,
            toml: TomlFileHandler,
        }
    }

    pub fn validate(self) -> Result<Self> {
        if !self.command_file_path.exists() {
            debug!(
                "Creating a new commands.toml file at {:?}",
                self.command_file_path
            );
            self.save(&hashmap!())?;
        } else {
            debug!("Found a commands.toml file at {:?}", self.command_file_path)
        }
        Ok(self)
    }

    pub fn save(&self, commands: &CommandMap) -> Result<(), FileError> {
        let toml = self.toml.generate_file_from_commands(commands)?;
        write!(self.command_file_path.as_path(), toml)
    }

    pub fn save_at<P>(&self, commands: &CommandMap, path: P) -> Result<(), FileError>
    where
        P: AsRef<Path>,
    {
        let toml = self.toml.generate_file_from_commands(commands)?;
        write!(path.as_ref(), toml)
    }

    pub fn load(&self) -> Result<CommandMap> {
        self.toml
            .generate_commands_from_file(&self.command_file_path)
    }

    pub fn load_from<P>(&self, path: P) -> Result<CommandMap>
    where
        P: AsRef<Path>,
    {
        self.toml.generate_commands_from_file(path)
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    // use anyhow::Result;
    // use std::{env::temp_dir, fs};

    // #[test]
    // fn should_save_a_file() -> Result<()> {
    //     let content = vec![Command::default()];
    //     let dir = temp_dir();
    //     let path = dir.join("test.toml");
    //     let file_service = CommandsFileHandler::new(path.to_owned()).validate()?;

    //     let result = file_service.save(&content);
    //     assert!(result.is_ok());
    //     let read_result = std::fs::read_to_string(path.as_path())?;

    //     assert!(!read_result.is_empty());

    //     fs::remove_file(path)?;
    //     Ok(())
    // }

    // #[test]
    // fn should_return_an_error() -> Result<()> {
    //     let dir = temp_dir();
    //     let path = dir.join("nonexistent/test.toml"); // .save() should not create any dir, so it will fail
    //     let result = CommandsFileHandler::new(path.to_owned()).validate();

    //     assert!(result.is_err());

    //     fs::remove_file(path).unwrap_or_else(|_| {});
    //     Ok(())
    // }
}
