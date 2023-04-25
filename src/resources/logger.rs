use crate::resources::config::LogLevel;
use anyhow::Result;
use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
use std::path::PathBuf;

pub fn init(log_level: LogLevel, log_path: PathBuf) -> Result<()> {
    let log_level_string = String::from(&log_level);
    Logger::try_with_str(log_level_string)?
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

    Ok(())
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
