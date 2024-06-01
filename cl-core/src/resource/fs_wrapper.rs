use super::errors::FileError;
use anyhow::Result;
use std::{fs, path::Path};

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
            $crate::resource::fs_wrapper::write($path, $contents)
        };
    }

    #[macro_export]
    macro_rules! read_to_string {
        ($path:expr) => {
            $crate::resource::fs_wrapper::read_to_string($path)
        };
    }

    #[macro_export]
    macro_rules! create_dir_all {
        ($path:expr) => {
            $crate::resource::fs_wrapper::create_dir_all($path)
        };
    }

    pub use create_dir_all;
    pub use read_to_string;
    pub use write;
}

pub use macros::{create_dir_all, read_to_string, write};
