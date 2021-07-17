use chrono::Utc;
use serde::Serialize;

pub trait Logger {
    fn error(&self, message: impl LogMessage);
    fn audit(&self, message: impl LogMessage);
    fn info(&self, message: impl LogMessage);
    fn debug(&self, message: impl LogMessage);

    fn log(&self, log_level: LogLevel, message: impl LogMessage) {
        match log_level {
            LogLevel::Error => self.error(message),
            LogLevel::Audit => self.audit(message),
            LogLevel::Info => self.info(message),
            LogLevel::Debug => self.debug(message),
        }
    }
}

pub trait LogMessage {
    fn log_message(&self) -> String;
}

pub enum LogLevel {
    Error,
    Audit,
    Info,
    Debug,
}

#[derive(Debug, Serialize)]
pub struct LogEntry<R: std::fmt::Debug + Serialize> {
    at: String,
    level: &'static str,
    message: String,
    request: R,
}

impl<R: std::fmt::Debug + Serialize> LogEntry<R> {
    pub fn with_message(level: &'static str, message: impl LogMessage, request: R) -> Self {
        Self {
            at: Utc::now().to_rfc3339(),
            level,
            message: message.log_message(),
            request,
        }
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(_) => format!("{:?}", self),
        }
    }
}
