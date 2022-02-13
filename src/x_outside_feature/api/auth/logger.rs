use serde::Serialize;

use crate::z_lib::logger::init::{InfoLogger, QuietLogger, VerboseLogger};

use crate::z_lib::logger::infra::Logger;

pub fn app_logger(target: &'static str, id: String) -> impl Logger {
    // アプリケーション全体で使用するデフォルトの logger を返す
    // 個別のアクションでレベルを指定したい時はそれぞれ個別のやつを呼び出す
    verbose_logger(target, id)
}
pub fn quiet_logger(target: &'static str, id: String) -> impl Logger {
    QuietLogger::with_request(RequestEntry::new(target, id))
}
pub fn info_logger(target: &'static str, id: String) -> impl Logger {
    InfoLogger::with_request(RequestEntry::new(target, id))
}
pub fn verbose_logger(target: &'static str, id: String) -> impl Logger {
    VerboseLogger::with_request(RequestEntry::new(target, id))
}

#[derive(Debug, Clone, Serialize)]
struct RequestEntry {
    id: String,
    target: &'static str,
}

impl RequestEntry {
    fn new(target: &'static str, id: String) -> Self {
        Self { id, target }
    }
}
