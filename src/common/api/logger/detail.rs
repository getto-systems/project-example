use chrono::Utc;
use serde::Serialize;

use crate::common::api::request::data::RequestInfo;

pub struct StdoutJsonLogger {
    request: RequestInfo,
}

impl StdoutJsonLogger {
    pub fn with_request(request: RequestInfo) -> Self {
        Self { request }
    }

    pub fn fatal(&self, message: String) {
        println!("{}", self.message(LogLevel::Fatal, message))
    }
    pub fn info(&self, message: String) {
        println!("{}", self.message(LogLevel::Info, message))
    }
    pub fn debug(&self, message: String) {
        println!("{}", self.message(LogLevel::Debug, message))
    }
    pub fn incident(&self, message: String) {
        println!("{}", self.message(LogLevel::Incident, message))
    }
    pub fn audit(&self, message: String) {
        println!("{}", self.message(LogLevel::Audit, message))
    }

    fn message(&self, level: LogLevel, message: String) -> String {
        LogEntry::new(level, self.request.clone(), message).to_json_or_debug_format()
    }
}

#[derive(Clone, Copy)]
enum LogLevel {
    Fatal,
    Info,
    Debug,
    Incident,
    Audit,
}

impl LogLevel {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Fatal => "FATAL",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Incident => "INCIDENT",
            Self::Audit => "AUDIT",
        }
    }
}

#[derive(Debug, Serialize)]
struct LogEntry {
    at: String,
    level: &'static str,
    request: RequestInfo,
    message: String,
}

impl LogEntry {
    fn new(level: LogLevel, request: RequestInfo, message: String) -> Self {
        Self {
            at: Utc::now().to_rfc3339(),
            level: level.as_str(),
            request,
            message,
        }
    }

    fn to_json_or_debug_format(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(_) => format!("{:?}", self),
        }
    }
}
