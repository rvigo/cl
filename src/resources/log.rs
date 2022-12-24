use anyhow::Result;
use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};

pub fn init() -> Result<()> {
    Logger::try_with_str("info")?
        .log_to_file(FileSpec::default().basename("output").directory("./log"))
        .append()
        .rotate(
            Criterion::Size(1024 * 1000 * 1),
            Naming::Numbers,
            Cleanup::KeepLogFiles(3),
        )
        .format_for_files(flexi_logger::detailed_format)
        .start()?;

    Ok(())
}
