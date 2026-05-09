//! This module contains the configuration options for the application.
//! # Examples:
//! ```
//! use cli_utils::config::Logging;
//! let config = Logging::new();
//! ```
//!
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// This enum represents the possible destinations for log output.
/// # Examples:
/// ```
/// use cli_utils::config::LogOutput;
/// let output = LogOutput::Stdout;
/// ```
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}

/// This struct contains configuration options for the application.
/// # Examples:
/// ```
/// use cli_utils::config::Logging;
/// let config = Logging::new();
/// ```
///
/// Creating a new instance of the Logging struct:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// ```
/// 
/// Getting the values of the Logging struct:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let config = Logging{ enabled: true, level: LogLevel::Info, destination: LogOutput::Stdout };
/// let (enabled, level, destination) = config.getter();
/// ```
/// Setting the values of the Logging struct:
/// ```
/// use cli_utils::config::{Logging, LogLevel, LogOutput};
/// let mut config = Logging::new();
/// config.setter(true, LogLevel::Debug, LogOutput::Stderr);
/// ```
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }

    pub fn getter(&self) -> (bool, &LogLevel, &LogOutput) {
        (self.enabled, &self.level, &self.destination)
    }

    pub fn setter(&mut self, enabled: bool, level: LogLevel, destination: LogOutput) {
        self.enabled = enabled;
        self.level = level;
        self.destination = destination;
    }
}
