use crate::resources::config::LogLevel;
use anyhow::Result;
use std::path::Path;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{
    fmt::format::{Format, PrettyFields},
    prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

pub fn init<P>(level: LogLevel, path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    tracing_logger_stdout_and_file(level, path.as_ref().join("log"))?;

    self::panic_handler::setup_panic_hook();
    Ok(())
}

/// Sets a logger with two layers (stdout and a file)
pub fn tracing_logger_stdout_and_file<L, P>(level: L, path: P) -> Result<()>
where
    L: Into<LevelFilter>,
    P: AsRef<Path>,
{
    let level_filter: LevelFilter = level.into();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .event_format(Format::default().with_source_location(false).without_time())
                .fmt_fields(PrettyFields::new())
                .with_target(false)
                .with_filter(
                    // ensures at least an info message to console
                    if level_filter == LevelFilter::ERROR {
                        LevelFilter::INFO
                    } else {
                        level_filter
                    },
                ),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(tracing_appender::rolling::never(path, "log.log"))
                .with_ansi(false)
                .with_thread_names(true)
                .with_filter(level_filter),
        )
        .init();

    Ok(())
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
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
        message.push_str("cause:\n");
        for line in payload.lines() {
            message.push_str(&format!("    {line}\n"))
        }
        error!("{message}")
    }
}

pub mod interceptor {
    use log::error;
    use std::{
        fmt::{Debug, Display},
        panic::Location,
    };

    /// Logs Error variant of Result enum if there is an error
    pub trait ErrorInterceptor<T, E> {
        fn log_error(self) -> Result<T, E>;
    }

    impl<T, E> ErrorInterceptor<T, E> for Result<T, E>
    where
        E: Display + Debug,
        Result<T, E>: anyhow::Context<T, E>,
    {
        /// Logs the error content if any
        #[inline]
        #[track_caller]
        fn log_error(self) -> Result<T, E> {
            match self {
                Ok(ok) => Ok(ok),
                Err(err) => {
                    let caller = Location::caller();
                    let record = ErrorRecord::new(
                        err.to_string(),
                        format!("{}:{}", caller.file(), caller.line()),
                        &err,
                    );

                    error!("{:#?}", record);
                    Err(err)
                }
            }
        }
    }

    struct ErrorRecord<E> {
        _message: String,
        _location: String,
        _stacktrace: E,
    }

    impl<E> ErrorRecord<E>
    where
        E: Debug,
    {
        pub fn new(message: String, location: String, stacktrace: E) -> ErrorRecord<E> {
            Self {
                _message: message,
                _location: location,
                _stacktrace: stacktrace,
            }
        }
    }

    impl<E> Debug for ErrorRecord<E>
    where
        E: Debug,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("ErrorRecord")
                .field("message", &self._message)
                .field("location", &self._location)
                .field("stacktrace", &self._stacktrace)
                .finish()
        }
    }
}
