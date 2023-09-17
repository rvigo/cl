use std::path::PathBuf;
use thiserror::Error;

/// Command related errors
#[derive(Error, Debug)]
pub enum CommandError {
    #[error("The alias must not contain whitespace as the application may interpret some words as arguments")]
    AliasWithWhitespaces,
    #[error("The namespace must not contain whitespace as the application may interpret some words as arguments")]
    NamespaceWithWhitespaces,
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
    CreateTomlFromString(#[from] toml::ser::Error),
    #[error("Cannot read {path}")]
    ReadFile {
        path: PathBuf,
        #[source]
        cause: anyhow::Error,
    },
    #[error("Cannot write {path}")]
    WriteFile {
        path: PathBuf,
        #[source]
        cause: anyhow::Error,
    },
    #[error("Cannot create dirs at {path}")]
    CreateDirs {
        path: PathBuf,
        #[source]
        cause: std::io::Error,
    },
}

pub(super) mod panic_handler {
    use log::error;
    use std::panic::PanicInfo;

    pub fn setup_panic_hook() {
        std::panic::set_hook(Box::new(format_panic_message));
    }

    fn format_panic_message(panic_info: &PanicInfo) {
        let mut message = String::from("The application crashed\n");
        let payload = panic_info
            .payload()
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| panic_info.payload().downcast_ref::<&str>().cloned())
            .unwrap_or("Box<Any>");
        message.push_str("Cause:\n");
        for line in payload.lines() {
            message.push_str(&format!("    {line}\n"))
        }
        error!("{message}")
    }
}
