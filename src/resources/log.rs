use crate::resources::config::{Config, LogLevel};
use anyhow::{Context, Result};
use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
use lazy_static::lazy_static;
use log::debug;
use std::sync::Mutex;

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(
        Config::load()
            .context("Cannot properly load the app configs")
            .unwrap()
    );
}

pub fn init() -> Result<()> {
    let config = CONFIG.lock().unwrap();
    let log_level = String::from(config.get_log_level()?.unwrap_or(&LogLevel::default()));
    Logger::try_with_str(&log_level)?
        .log_to_file(
            FileSpec::default()
                .basename("output")
                .directory(format!("{}/log", config.get_app_home_dir().display())),
        )
        .append()
        .rotate(
            Criterion::Size(1024 * 1000),
            Naming::Numbers,
            Cleanup::KeepLogFiles(3),
        )
        .format_for_files(flexi_logger::detailed_format)
        .start()?;

    debug!("log started ok with log_level `{}`", &log_level);
    Ok(())
}
