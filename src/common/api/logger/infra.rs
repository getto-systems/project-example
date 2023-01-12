pub trait Logger {
    fn log(&self, message: &(impl LogFilter + LogMessage));
}
pub trait LogFilter {
    fn log_level(&self) -> LogLevel;
}
pub trait LogMessage {
    fn log_message(&self) -> String;
}

pub enum LogLevel {
    Error,
    Important,
    Info,
    Debug,
}

impl LogLevel {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "ERROR",
            Self::Important => "IMPORTANT",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
        }
    }
}

#[derive(Clone, Copy)]
pub enum LogOutputLevel {
    Quiet,
    Info,
    Verbose,
}

impl LogOutputLevel {
    pub fn parse(level: &'static str) -> Self {
        match level {
            "quiet" => Self::Quiet,
            "info" => Self::Info,
            "verbose" => Self::Verbose,
            _ => Self::default(),
        }
    }
}
