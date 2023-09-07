use crate::resources::config::LogLevel;
use anyhow::Result;
use std::path::Path;
use tracing::{metadata::LevelFilter, Subscriber};
use tracing_appender::rolling;
use tracing_subscriber::{
    fmt::{
        self,
        format::{Format, PrettyFields},
    },
    prelude::__tracing_subscriber_SubscriberExt,
    registry::LookupSpan,
    util::SubscriberInitExt,
    Layer,
};

#[derive(Default)]
pub enum LoggerType {
    #[default]
    MainApp,
    Subcommand,
}

#[derive(Default)]
pub struct LoggerBuilder<P> {
    log_level: LogLevel,
    logger_type: LoggerType,
    path: P,
}

impl<P> LoggerBuilder<P>
where
    P: AsRef<Path>,
{
    pub fn with_log_level(mut self, log_level: LogLevel) -> LoggerBuilder<P> {
        self.log_level = log_level;
        self
    }

    pub fn with_path(mut self, path: P) -> LoggerBuilder<P> {
        self.path = path;
        self
    }

    pub fn with_logger_type(mut self, logger_type: LoggerType) -> LoggerBuilder<P> {
        self.logger_type = logger_type;
        self
    }

    pub fn build(self) -> Logger<P> {
        Logger {
            log_level: self.log_level,
            logger_type: self.logger_type,
            path: self.path,
        }
    }
}

pub struct Logger<P> {
    log_level: LogLevel,
    logger_type: LoggerType,
    path: P,
}

impl<P> Logger<P>
where
    P: AsRef<Path>,
{
    pub fn init(&self) -> Result<()> {
        match self.logger_type {
            LoggerType::MainApp => self.init_main_app_logger()?,
            LoggerType::Subcommand => self.init_subcommand_logger()?,
        }

        self::panic_handler::setup_panic_hook();
        Ok(())
    }

    /// Sets a logger with a single layer
    fn init_main_app_logger(&self) -> Result<()> {
        let level_filter: LevelFilter = self.log_level.to_owned().into();
        tracing_subscriber::registry()
            .with(self.get_logfile_layer(level_filter))
            .init();

        Ok(())
    }

    /// Sets a logger with two layers (stdout and a file)
    fn init_subcommand_logger(&self) -> Result<()> {
        let level_filter: LevelFilter = self.log_level.to_owned().into();
        tracing_subscriber::registry()
            .with(self.get_stdout_layer(level_filter))
            .with(self.get_logfile_layer(level_filter))
            .init();

        Ok(())
    }

    fn get_logfile_layer<S>(&self, level_filter: LevelFilter) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
        P: AsRef<Path>,
    {
        fmt::layer()
            .with_writer(rolling::daily(self.path.as_ref(), "log.log"))
            .with_ansi(false)
            .with_filter(level_filter)
    }

    fn get_stdout_layer<S>(&self, level_filter: LevelFilter) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        fmt::layer()
            .event_format(Format::default().with_source_location(false).without_time())
            .fmt_fields(PrettyFields::new())
            .with_target(false)
            .with_filter(
                // ensures at least INFO messages when logging to console
                if level_filter == LevelFilter::ERROR {
                    LevelFilter::INFO
                } else {
                    level_filter
                },
            )
    }
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
