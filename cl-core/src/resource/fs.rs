use super::errors::FileError;
use crate::{resource::toml::Toml, CommandMap};
use anyhow::Result;
use std::{fs, path::Path};

pub fn save_at<P>(commands: &CommandMap, path: P) -> Result<(), FileError>
where
    P: AsRef<Path>,
{
    let toml = Toml::from_map(commands)?;
    write!(path.as_ref(), toml)
}

pub fn load_from<'f, P>(path: P) -> Result<CommandMap<'f>>
where
    P: AsRef<Path>,
{
    Toml::from_file(path)
}

pub fn write<P, C>(path: P, contents: C) -> Result<(), FileError>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    fs::write(&path, contents).map_err(|cause| FileError::WriteFile {
        path: path.as_ref().to_path_buf(),
        cause: cause.into(),
    })
}

pub fn read_to_string<P>(path: P) -> Result<String, FileError>
where
    P: AsRef<Path>,
{
    fs::read_to_string(&path).map_err(|cause| FileError::ReadFile {
        path: path.as_ref().to_path_buf(),
        cause: cause.into(),
    })
}

pub fn create_dir_all<P>(path: P) -> Result<(), FileError>
where
    P: AsRef<Path>,
{
    std::fs::create_dir_all(&path).map_err(|cause| FileError::CreateDirs {
        path: path.as_ref().to_path_buf(),
        cause,
    })
}

mod macros {
    #[macro_export]
    macro_rules! write {
        ($path:expr, $contents:expr) => {
            $crate::resource::fs::write($path, $contents)
        };
    }

    #[macro_export]
    macro_rules! read_to_string {
        ($path:expr) => {
            $crate::resource::fs::read_to_string($path)
        };
    }

    #[macro_export]
    macro_rules! create_dir_all {
        ($path:expr) => {
            $crate::resource::fs::create_dir_all($path)
        };
    }

    pub use create_dir_all;
    pub use read_to_string;
    pub use write;
}

pub use macros::{create_dir_all, read_to_string, write};

#[cfg(test)]
mod test {
    // use super::*;
    // use crate::{command::Command, CommandVecExt};
    // use anyhow::Result;
    // use tempdir::TempDir;

    // #[test]
    // fn should_save_a_file() -> Result<()> {
    //     let content = vec![Command::default()];
    //     let dir = TempDir::new("handler")?;
    //     let path = dir.path().join("test.toml");
    //     let file_service = FileService::new(path.to_owned())?;

    //     let result = file_service.save(&content.to_command_map());
    //     assert!(result.is_ok());
    //     let read_result = std::fs::read_to_string(path.as_path())?;

    //     assert!(!read_result.is_empty());

    //     Ok(())
    // }

    // #[test]
    // fn should_return_an_error() -> Result<()> {
    //     let dir = TempDir::new("handler")?;
    //     let path = dir.path().join("nonexistent/test.toml"); // .save() should not create any dir, so it will fail
    //     let result = FileService::new(path.to_owned());

    //     assert!(result.is_err());

    //     Ok(())
    // }
}
