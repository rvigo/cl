use anyhow::{anyhow, Result};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};

pub fn init() -> Result<()> {
    let level = log::LevelFilter::Info;
    let log_file_path = ".config/command_list/log/log.log";
    let home_dir = match dirs::home_dir() {
        Some(home) => home,
        None => return Err(anyhow!("No $HOME directory found for config.")),
    };

    let full_log_path = home_dir.join(log_file_path);

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S %Z)(utc)} [{f}:{L}] {M}:{m}{n}",
        )))
        .build(full_log_path)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(level))
        .unwrap();

    let _handle = log4rs::init_config(config)?;

    Ok(())
}
