use super::{fs_wrapper::write, toml::Toml};
use crate::{hashmap, resource::errors::FileError, CommandMap};
use anyhow::Result;
use log::debug;
use std::path::{Path, PathBuf};

pub struct FileService {
    file_path: PathBuf,
}

impl FileService {
    pub fn new(file_path: PathBuf) -> Result<FileService> {
        let service = Self { file_path };
        service.validate()
    }

    pub fn save(&self, commands: &CommandMap) -> Result<(), FileError> {
        let toml = Toml::from_map(commands)?;
        write!(self.file_path.as_path(), toml)
    }

    pub fn save_at<P>(&self, commands: &CommandMap, path: P) -> Result<(), FileError>
    where
        P: AsRef<Path>,
    {
        let toml = Toml::from_map(commands)?;
        write!(path.as_ref(), toml)
    }

    pub fn load(&self) -> Result<CommandMap> {
        Toml::from_file(&self.file_path)
    }

    pub fn load_from<P>(&self, path: P) -> Result<CommandMap>
    where
        P: AsRef<Path>,
    {
        Toml::from_file(path)
    }

    fn validate(self) -> Result<Self> {
        if !self.file_path.exists() {
            debug!("Creating a new commands.toml file at {:?}", self.file_path);
            self.save(&hashmap!())?;
        }
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{command::Command, CommandVecExt};
    use anyhow::Result;
    use tempdir::TempDir;

    #[test]
    fn should_save_a_file() -> Result<()> {
        let content = vec![Command::default()];
        let dir = TempDir::new("handler")?;
        let path = dir.path().join("test.toml");
        let file_service = FileService::new(path.to_owned())?;

        let result = file_service.save(&content.to_command_map());
        assert!(result.is_ok());
        let read_result = std::fs::read_to_string(path.as_path())?;

        assert!(!read_result.is_empty());

        Ok(())
    }

    #[test]
    fn should_return_an_error() -> Result<()> {
        let dir = TempDir::new("handler")?;
        let path = dir.path().join("nonexistent/test.toml"); // .save() should not create any dir, so it will fail
        let result = FileService::new(path.to_owned());

        assert!(result.is_err());

        Ok(())
    }
}
