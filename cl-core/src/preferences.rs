use crate::config::LogLevel;
use serde::{Deserialize, Serialize};

// defaults
const DEFAULT_LOG_LEVEL: &LogLevel = &LogLevel::Error;
const DEFAULT_QUIET_MODE: bool = false;
const DEFAULT_HIGHLIGHT_MATCHES: bool = true;

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Preferences {
    #[serde(skip_serializing_if = "Option::is_none", alias = "default_quiet_mode")]
    quiet_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "log_level")]
    log_level: Option<LogLevel>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "highlight_matches")]
    highlight_matches: Option<bool>,
}

impl Preferences {
    pub fn new() -> Preferences {
        Self {
            quiet_mode: None,
            log_level: None,
            highlight_matches: None,
        }
    }

    pub fn highlight(&self) -> bool {
        self.highlight_matches.unwrap_or(DEFAULT_HIGHLIGHT_MATCHES)
    }

    pub fn set_highlight(&mut self, highlight: bool) {
        self.highlight_matches = Some(highlight);
    }

    pub fn log_level(&self) -> LogLevel {
        self.log_level
            .as_ref()
            .unwrap_or(DEFAULT_LOG_LEVEL)
            .to_owned()
    }

    pub fn set_log_level(&mut self, log_level: LogLevel) {
        self.log_level = Some(log_level);
    }

    pub fn quiet_mode(&self) -> bool {
        self.quiet_mode.unwrap_or(DEFAULT_QUIET_MODE)
    }

    pub fn set_quiet_mode(&mut self, quiet_mode: bool) {
        self.quiet_mode = Some(quiet_mode);
    }
}

#[cfg(test)]
mod test {
    use crate::{config::LogLevel, preferences::Preferences};

    #[test]
    fn should_set_default_quiet_mode() {
        let mut preferences = Preferences::default();

        assert_eq!(preferences.quiet_mode(), false);

        preferences.set_quiet_mode(true);

        assert_eq!(preferences.quiet_mode(), true);
    }

    #[test]
    fn should_set_log_level() {
        let mut preferences = Preferences::default();

        assert_eq!(
            String::from(&preferences.log_level()),
            String::from(&LogLevel::Error)
        );

        preferences.set_log_level(LogLevel::Debug);

        assert_eq!(
            String::from(&preferences.log_level()),
            String::from(&LogLevel::Debug)
        );
    }

    #[test]
    fn should_set_highlight() {
        let mut preferences = Preferences::default();

        assert_eq!(preferences.highlight(), true);

        preferences.set_highlight(false);

        assert_eq!(preferences.highlight(), false);
    }
}
