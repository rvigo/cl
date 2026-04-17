use super::errors::FileError;
use crate::{resource::toml::Toml, CommandMap};
use anyhow::Result;
use std::{fs, path::Path};
use tracing::{debug, trace};

pub fn save_at<P>(commands: &CommandMap, path: P) -> Result<(), FileError>
where
    P: AsRef<Path>,
{
    debug!(target: "cl_core::fs", path = %path.as_ref().display(), "saving commands to file");
    let toml = Toml::from_map(commands)?;
    write!(path.as_ref(), toml)
}

pub fn load_from<'f, P>(path: P) -> Result<CommandMap<'f>>
where
    P: AsRef<Path>,
{
    debug!(target: "cl_core::fs", path = %path.as_ref().display(), "loading commands from file");
    Toml::from_file(path)
}

pub fn write<P, C>(path: P, contents: C) -> Result<(), FileError>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    trace!(target: "cl_core::fs", path = %path.as_ref().display(), "writing file");
    fs::write(&path, contents).map_err(|cause| FileError::WriteFile {
        path: path.as_ref().to_path_buf(),
        cause: cause.into(),
    })
}

pub fn read_to_string<P>(path: P) -> Result<String, FileError>
where
    P: AsRef<Path>,
{
    trace!(target: "cl_core::fs", path = %path.as_ref().display(), "reading file");
    fs::read_to_string(&path).map_err(|cause| FileError::ReadFile {
        path: path.as_ref().to_path_buf(),
        cause: cause.into(),
    })
}

pub fn create_dir_all<P>(path: P) -> Result<(), FileError>
where
    P: AsRef<Path>,
{
    trace!(target: "cl_core::fs", path = %path.as_ref().display(), "creating dirs");
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
    use super::*;
    use crate::{command::Command, CommandVecExt};
    use anyhow::Result;
    use tempfile::TempDir;

    #[test]
    fn should_save_a_file() -> Result<()> {
        let content = vec![Command::default()];
        let dir = TempDir::new()?;
        let path = dir.path().join("test.toml");

        save_at(&content.to_command_map(), &path)?;
        let read_result = std::fs::read_to_string(path.as_path())?;

        assert!(!read_result.is_empty());

        Ok(())
    }

    #[test]
    fn should_return_an_error_on_nonexistent_path() -> Result<()> {
        let dir = TempDir::new()?;
        let path = dir.path().join("nonexistent/test.toml");

        let content = vec![Command::default()];
        let result = save_at(&content.to_command_map(), &path);

        assert!(result.is_err());

        Ok(())
    }
}
