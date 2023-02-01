use anyhow::Result;
use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};

use super::config::CONFIG;

pub fn init() -> Result<()> {
    Logger::try_with_str("info")?
        .log_to_file(
            FileSpec::default()
                .basename("output")
                .directory(format!("{}/log", &CONFIG.get_app_home_dir().display())),
        )
        .append()
        .rotate(
            Criterion::Size(1024 * 1000),
            Naming::Numbers,
            Cleanup::KeepLogFiles(3),
        )
        .format_for_files(flexi_logger::detailed_format)
        .start()?;

    Ok(())
}
