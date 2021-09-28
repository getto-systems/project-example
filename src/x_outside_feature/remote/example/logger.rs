use serde::Serialize;

use crate::z_details::_common::logger::{InfoLogger, Logger, QuietLogger, VerboseLogger};

pub fn app_logger(target: &'static str, request_id: &str) -> impl Logger {
    // アプリケーション全体で使用するデフォルトの logger を返す
    // 個別のアクションでレベルを指定したい時はそれぞれ個別のやつを呼び出す
    verbose_logger(target, request_id)
}
pub fn quiet_logger(target: &'static str, request_id: &str) -> impl Logger {
    QuietLogger::with_request(RequestEntry::new(target, request_id))
}
pub fn info_logger(target: &'static str, request_id: &str) -> impl Logger {
    InfoLogger::with_request(RequestEntry::new(target, request_id))
}
pub fn verbose_logger(target: &'static str, request_id: &str) -> impl Logger {
    VerboseLogger::with_request(RequestEntry::new(target, request_id))
}

#[derive(Debug, Clone, Serialize)]
struct RequestEntry {
    id: String,
    target: &'static str,
}

impl RequestEntry {
    fn new(target: &'static str, request_id: &str) -> Self {
        Self {
            id: request_id.into(),
            target,
        }
    }
}
