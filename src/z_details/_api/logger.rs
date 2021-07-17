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

const ERROR: &'static str = "ERROR";
const AUDIT: &'static str = "AUDIT";
const INFO: &'static str = "INFO";
const DEBUG: &'static str = "DEBUG";

pub struct QuietLogger<R: std::fmt::Debug + Serialize + Clone> {
    request: R,
}

impl<R: std::fmt::Debug + Serialize + Clone> QuietLogger<R> {
    pub fn with_request(request: R) -> Self {
        Self { request }
    }

    fn message(&self, level: &'static str, message: impl LogMessage) -> String {
        log_message(level, message, self.request.clone())
    }
}

impl<R: std::fmt::Debug + Serialize + Clone> Logger for QuietLogger<R> {
    fn error(&self, message: impl LogMessage) {
        println!("{}", self.message(ERROR, message))
    }
    fn audit(&self, message: impl LogMessage) {
        println!("{}", self.message(AUDIT, message))
    }
    fn info(&self, _message: impl LogMessage) {
        // no log for info
    }
    fn debug(&self, _message: impl LogMessage) {
        // no log for debug
    }
}

pub struct InfoLogger<R: std::fmt::Debug + Serialize + Clone> {
    request: R,
}

impl<R: std::fmt::Debug + Serialize + Clone> InfoLogger<R> {
    pub fn with_request(request: R) -> Self {
        Self { request }
    }

    fn message(&self, level: &'static str, message: impl LogMessage) -> String {
        log_message(level, message, self.request.clone())
    }
}

impl<R: std::fmt::Debug + Serialize + Clone> Logger for InfoLogger<R> {
    fn error(&self, message: impl LogMessage) {
        println!("{}", self.message(ERROR, message))
    }
    fn audit(&self, message: impl LogMessage) {
        println!("{}", self.message(AUDIT, message))
    }
    fn info(&self, message: impl LogMessage) {
        println!("{}", self.message(INFO, message))
    }
    fn debug(&self, _message: impl LogMessage) {
        // no log for debug
    }
}

pub struct VerboseLogger<R: std::fmt::Debug + Serialize + Clone> {
    request: R,
}

impl<R: std::fmt::Debug + Serialize + Clone> VerboseLogger<R> {
    pub fn with_request(request: R) -> Self {
        Self { request }
    }

    fn message(&self, level: &'static str, message: impl LogMessage) -> String {
        log_message(level, message, self.request.clone())
    }
}

impl<R: std::fmt::Debug + Serialize + Clone> Logger for VerboseLogger<R> {
    fn error(&self, message: impl LogMessage) {
        println!("{}", self.message(ERROR, message))
    }
    fn audit(&self, message: impl LogMessage) {
        println!("{}", self.message(AUDIT, message))
    }
    fn info(&self, message: impl LogMessage) {
        println!("{}", self.message(INFO, message))
    }
    fn debug(&self, message: impl LogMessage) {
        println!("{}", self.message(DEBUG, message))
    }
}

fn log_message<R: std::fmt::Debug + Serialize + Clone>(
    level: &'static str,
    message: impl LogMessage,
    request: R,
) -> String {
    LogEntry::with_message(level, message, request).to_json()
}
