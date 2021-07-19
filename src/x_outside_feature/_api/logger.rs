use actix_web::HttpRequest;
use serde::Serialize;
use uuid::Uuid;

use crate::z_details::_common::logger::{InfoLogger, Logger, QuietLogger, VerboseLogger};

pub fn generate_request_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn app_logger(id: String, request: &HttpRequest) -> impl Logger {
    // アプリケーション全体で使用するデフォルトの logger を返す
    // 個別のアクションでレベルを指定したい時はそれぞれ個別のやつを呼び出す
    verbose_logger(id, request)
}
pub fn quiet_logger(id: String, request: &HttpRequest) -> impl Logger {
    QuietLogger::with_request(RequestEntry::new(id, request))
}
pub fn info_logger(id: String, request: &HttpRequest) -> impl Logger {
    InfoLogger::with_request(RequestEntry::new(id, request))
}
pub fn verbose_logger(id: String, request: &HttpRequest) -> impl Logger {
    VerboseLogger::with_request(RequestEntry::new(id, request))
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
