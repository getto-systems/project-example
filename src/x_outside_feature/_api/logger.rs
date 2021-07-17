use actix_web::HttpRequest;
use serde::Serialize;
use uuid::Uuid;

use crate::z_details::_api::logger::{LogEntry, LogMessage, Logger};

const ERROR: &'static str = "ERROR";
const AUDIT: &'static str = "AUDIT";
const INFO: &'static str = "INFO";
const DEBUG: &'static str = "DEBUG";

pub fn request_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn app_logger(id: String, request: &HttpRequest) -> impl Logger {
    // アプリケーション全体で使用するデフォルトの logger を返す
    // 個別のアクションでレベルを指定したい時はそれぞれ個別のやつを呼び出す
    verbose_logger(id, request)
}
pub fn quiet_logger(id: String, request: &HttpRequest) -> impl Logger {
    QuietLogger::with_request(id, request)
}
pub fn info_logger(id: String, request: &HttpRequest) -> impl Logger {
    InfoLogger::with_request(id, request)
}
pub fn verbose_logger(id: String, request: &HttpRequest) -> impl Logger {
    VerboseLogger::with_request(id, request)
}

pub struct QuietLogger {
    request: RequestEntry,
}

impl QuietLogger {
    fn with_request(id: String, request: &HttpRequest) -> Self {
        Self {
            request: RequestEntry::new(id, request),
        }
    }

    fn message(&self, level: &'static str, message: impl LogMessage) -> String {
        log_message(level, message, self.request.clone())
    }
}

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

pub struct InfoLogger {
    request: RequestEntry,
}

impl InfoLogger {
    fn with_request(id: String, request: &HttpRequest) -> Self {
        Self {
            request: RequestEntry::new(id, request),
        }
    }

    fn message(&self, level: &'static str, message: impl LogMessage) -> String {
        log_message(level, message, self.request.clone())
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

pub struct VerboseLogger {
    request: RequestEntry,
}

impl VerboseLogger {
    fn with_request(id: String, request: &HttpRequest) -> Self {
        Self {
            request: RequestEntry::new(id, request),
        }
    }

    fn message(&self, level: &'static str, message: impl LogMessage) -> String {
        log_message(level, message, self.request.clone())
    }
}

fn log_message(level: &'static str, message: impl LogMessage, request: RequestEntry) -> String {
    LogEntry::with_message(level, message, request).to_json()
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

#[derive(Debug, Clone, Serialize)]
struct RequestEntry {
    id: String,
    path: String,
    method: String,
}

impl RequestEntry {
    fn new(id: String, request: &HttpRequest) -> Self {
        Self {
            id,
            path: request.path().to_string(),
            method: request.method().to_string(),
        }
    }
}
