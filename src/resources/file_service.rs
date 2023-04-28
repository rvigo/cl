use crate::{command::Command, resources::errors::FileError};
use anyhow::{Context, Result};
use log::debug;
use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    path::{Path, PathBuf},
};

pub struct FileService {
    command_file_path: PathBuf,
}

impl FileService {
    pub fn new(command_file_path: PathBuf) -> FileService {
        Self { command_file_path }
    }

    pub fn save_file(&self, contents: &str, path: &Path) -> Result<(), FileError> {
        write(path, contents).map_err(|cause| FileError::WriteFile {
            file_path: self.command_file_path.to_path_buf(),
            cause: cause.into(),
        })
    }

    pub fn open_file(&self, path: &Path) -> Result<String, FileError> {
        let path_str = path.display().to_string();

        read_to_string(path_str).map_err(|cause| FileError::ReadFile {
            file_path: path.to_path_buf(),
            cause: cause.into(),
        })
    }

    pub fn convert_from_toml_file(&self, path: &Path) -> Result<Vec<Command>> {
        let toml = toml::from_str::<HashMap<String, Vec<Command>>>(&self.open_file(path)?)?;
        let mut commands: Vec<Command> = toml
            .into_iter()
            .flat_map(|(_, _commands)| _commands)
            .collect();
        commands.sort();
        Ok(commands)
    }

    pub fn load_commands_from_file(&self) -> Result<Vec<Command>> {
        self.convert_from_toml_file(&self.command_file_path)
    }

    fn generate_toml(&self, commands: &Vec<Command>) -> Result<String, FileError> {
        let mut map: HashMap<String, Vec<Command>> = HashMap::new();
        for command in commands {
            let item = command.to_owned();
            if let Some(commands) = map.get_mut(&item.namespace) {
                commands.push(item);
            } else {
                map.insert(item.namespace.to_owned(), vec![item]);
            }
        }

        toml::to_string(&map).map_err(FileError::from)
    }

    pub fn write_toml_file(&self, commands: &Vec<Command>, path: &Path) -> Result<(), FileError> {
        let toml = self.generate_toml(commands)?;
        self.save_file(&toml, path)
    }

    pub fn write_to_command_file(&self, commands: &Vec<Command>) -> Result<(), FileError> {
        let path = &self.command_file_path;
        debug!(
            "saving {} commands to {:?}",
            commands.len(),
            self.command_file_path
        );
        self.write_toml_file(commands, path)
            .context("Failed to write to the commands file")
            .map_err(|cause| FileError::WriteFile {
                file_path: self.command_file_path.to_owned(),
                cause,
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        env::temp_dir,
        fs::{self, File},
        io::Read,
    };

    #[test]
    fn should_save_a_file() {
        let content = "Some content";
        let dir = temp_dir();
        let path = dir.join("test.toml");
        let file_service = FileService::new(path.clone());

        let result = file_service.save_file(content, &path);

        assert!(result.is_ok());
        let mut file = File::open(&path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).unwrap();
        assert_eq!(file_contents, content);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn should_return_an_error() {
        let content = "Some content";
        let dir = temp_dir();
        let path = dir.join("nonexistent/test.toml");
        let file_service = FileService::new(path.clone());

        let result = file_service.save_file(content, &path);

        assert!(result.is_err());

        fs::remove_file(path).unwrap_or_else(|_| {});
    }
}
