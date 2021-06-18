use std::fmt::{Display, Formatter};

use actix_web::HttpRequest;
use chrono::Utc;
use serde::Serialize;
use uuid::Uuid;

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

pub fn app_logger(request: &HttpRequest) -> impl Logger {
    // アプリケーション全体でしようするデフォルトの logger を返す
    // 個別のアクションでレベルを指定した logger を使用することもできる
    verbose_logger(request)
}

pub fn quiet_logger(request: &HttpRequest) -> impl Logger {
    QuietLogger::with_request(request)
}

pub fn info_logger(request: &HttpRequest) -> impl Logger {
    InfoLogger::with_request(request)
}

pub fn verbose_logger(request: &HttpRequest) -> impl Logger {
    VerboseLogger::with_request(request)
}

pub struct QuietLogger {
    request: RequestEntry,
}

impl QuietLogger {
    fn with_request(request: &HttpRequest) -> Self {
        Self {
            request: RequestEntry::new(request),
        }
    }

    fn message(&self, level: &'static str, message: impl LogMessage) -> String {
        LogEntry::with_message(level, message, self.request.clone()).to_json()
    }
}

pub struct InfoLogger {
    request: RequestEntry,
}

impl InfoLogger {
    fn with_request(request: &HttpRequest) -> Self {
        Self {
            request: RequestEntry::new(request),
        }
    }

    fn message(&self, level: &'static str, message: impl LogMessage) -> String {
        LogEntry::with_message(level, message, self.request.clone()).to_json()
    }
}

pub struct VerboseLogger {
    request: RequestEntry,
}

impl VerboseLogger {
    fn with_request(request: &HttpRequest) -> Self {
        Self {
            request: RequestEntry::new(request),
        }
    }

    fn message(&self, level: &'static str, message: impl LogMessage) -> String {
        LogEntry::with_message(level, message, self.request.clone()).to_json()
    }
}

const ERROR: &'static str = "ERROR";
const AUDIT: &'static str = "AUDIT";
const INFO: &'static str = "INFO";
const DEBUG: &'static str = "DEBUG";

impl Logger for QuietLogger {
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

impl Logger for InfoLogger {
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

impl Logger for VerboseLogger {
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

#[derive(Serialize)]
struct LogEntry {
    at: String,
    level: &'static str,
    message: String,
    request: RequestEntry,
}

impl LogEntry {
    fn with_message(level: &'static str, message: impl LogMessage, request: RequestEntry) -> Self {
        Self {
            at: Utc::now().to_rfc3339(),
            level,
            message: message.log_message(),
            request,
        }
    }

    fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(_) => format!("{}", self),
        }
    }
}

impl Display for LogEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}: {}: {}: {}",
            self.at, self.level, self.message, self.request
        )
    }
}

#[derive(Clone, Serialize)]
struct RequestEntry {
    id: String,
    // TODO remote addr と host はこの時点では意味ないからなくす; cloud run のログとかで確認できる
    remote_addr: Option<String>,
    host: String,
    path: String,
    method: String,
}

impl RequestEntry {
    fn new(request: &HttpRequest) -> Self {
        let connection_info = request.connection_info();
        Self {
            id: Uuid::new_v4().to_string(),
            remote_addr: connection_info.remote_addr().map(|addr| addr.to_string()),
            host: connection_info.host().to_string(),
            path: request.path().to_string(),
            method: request.method().to_string(),
        }
    }
}

impl Display for RequestEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let remote_addr = match self.remote_addr.clone() {
            Some(addr) => addr,
            None => "X.X.X.X".to_string(),
        };
        write!(
            f,
            "{}: {}: {}, {}",
            self.id, remote_addr, self.path, self.method
        )
    }
}
