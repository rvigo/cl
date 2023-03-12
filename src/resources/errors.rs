use std::path::PathBuf;
use thiserror::Error;

/// Command related errors
#[derive(Error, Debug)]
pub enum CommandError {
    #[error("The alias must not contain whitespace as the application may interpret some words as arguments")]
    AliasWithWhitespaces,
    #[error("The alias \'{alias}\' was not found!")]
    AliasNotFound { alias: String },
    #[error("Command with alias \'{alias}\' already exists in \'{namespace}\' namespace")]
    CommandAlreadyExists { alias: String, namespace: String },
    #[error(
        "There are commands with the alias \'{alias}\' in multiples namespaces. \
    Please use the \'--namespace\' flag"
    )]
    CommandPresentInManyNamespaces { alias: String },
    #[error("Cannot run the command '{command}'\n\nCause: {cause}")]
    CannotRunCommand { command: String, cause: String },
    #[error("Namespace, command and alias field cannot be empty!")]
    EmptyCommand,
}

/// File related errors
#[derive(Error, Debug)]
pub enum FileError {
    #[error("Cannot create a String of TOML")]
    CannotCreateTomlFromString(#[from] toml::ser::Error),
    #[error("Cannot read {file_path}")]
    CannotReadFile {
        file_path: PathBuf,
        #[source]
        cause: anyhow::Error,
    },
    #[error("Cannot write {file_path}")]
    CannotWriteFile {
        file_path: PathBuf,
        #[source]
        cause: anyhow::Error,
    },
}
