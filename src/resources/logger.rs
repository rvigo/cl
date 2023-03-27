use crate::resources::config::LogLevel;
use anyhow::Result;
use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
use log::{debug, error};
use std::path::PathBuf;

pub fn init(log_level: LogLevel, log_path: PathBuf) -> Result<()> {
    let log_level_string = String::from(&log_level);
    Logger::try_with_str(&log_level_string)?
        .log_to_file(
            FileSpec::default()
                .basename("output")
                .directory(log_path.join("log")),
        )
        .append()
        .rotate(
            Criterion::Size(1024 * 1000),
            Naming::Numbers,
            Cleanup::KeepLogFiles(3),
        )
        .format_for_files(flexi_logger::detailed_format)
        .start()?;

    debug!("log started ok with log_level `{}`", &log_level_string);
    Ok(())
}

pub trait ErrorInterceptor<T> {
    fn log_if_error(self) -> Result<T>;
}

impl<T> ErrorInterceptor<T> for Result<T> {
    fn log_if_error(self) -> Result<T> {
        if let Err(ref err) = self {
            error!("{err:?}");
        }
        self
    }
}