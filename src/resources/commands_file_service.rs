use super::{fs_wrapper, toml::TomlFileHandler};
use crate::{command::Command, resources::errors::FileError};
use anyhow::Result;
use log::debug;
use std::path::{Path, PathBuf};

pub struct CommandsFileService {
    command_file_path: PathBuf,
}

impl TomlFileHandler for CommandsFileService {}

impl CommandsFileService {
    pub fn new(command_file_path: PathBuf) -> CommandsFileService {
        Self { command_file_path }
    }

    pub fn validate(self) -> Result<Self> {
        if !self.command_file_path.exists() {
            debug!(
                "Creating a new commands.toml file at {:?}",
                self.command_file_path
            );
            self.save(&vec![])?;
        } else {
            debug!("Found a commands.toml file at {:?}", self.command_file_path)
        }
        Ok(self)
    }

    pub fn save(&self, commands: &Vec<Command>) -> Result<(), FileError> {
        let toml = self.generate_toml_from_commands(commands)?;
        fs_wrapper::write(self.command_file_path.as_path(), toml)
    }

    pub fn save_at<P>(&self, commands: &Vec<Command>, path: P) -> Result<(), FileError>
    where
        P: AsRef<Path>,
    {
        let toml = self.generate_toml_from_commands(commands)?;
        fs_wrapper::write(path.as_ref(), toml)
    }

    pub fn load(&self) -> Result<Vec<Command>> {
        self.generate_commands_from_toml(&self.command_file_path)
    }

    pub fn load_from<P>(&self, path: P) -> Result<Vec<Command>>
    where
        P: AsRef<Path>,
    {
        self.generate_commands_from_toml(path)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use std::{env::temp_dir, fs};

    #[test]
    fn should_save_a_file() -> Result<()> {
        let content = vec![Command::default()];
        let dir = temp_dir();
        let path = dir.join("test.toml");
        let file_service = CommandsFileService::new(path.clone()).validate()?;

        let result = file_service.save(&content);
        assert!(result.is_ok());
        let read_result = std::fs::read_to_string(path.as_path())?;

        assert!(!read_result.is_empty());

        fs::remove_file(path)?;
        Ok(())
    }

    #[test]
    fn should_return_an_error() -> Result<()> {
        let dir = temp_dir();
        let path = dir.join("nonexistent/test.toml"); // .save() should not create any dir, so it will fail
        let result = CommandsFileService::new(path.clone()).validate();

        assert!(result.is_err());

        fs::remove_file(path).unwrap_or_else(|_| {});
        Ok(())
    }
}
