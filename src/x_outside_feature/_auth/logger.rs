use serde::Serialize;
use tonic::Request;

use crate::x_outside_feature::_common::metadata::METADATA_REQUEST_ID;

use crate::z_details::_common::logger::{InfoLogger, Logger, QuietLogger, VerboseLogger};

pub fn app_logger<T>(target: &'static str, request: &Request<T>) -> impl Logger {
    // アプリケーション全体で使用するデフォルトの logger を返す
    // 個別のアクションでレベルを指定したい時はそれぞれ個別のやつを呼び出す
    verbose_logger(target, request)
}
pub fn quiet_logger<T>(target: &'static str, request: &Request<T>) -> impl Logger {
    QuietLogger::with_request(RequestEntry::new(target, request))
}
pub fn info_logger<T>(target: &'static str, request: &Request<T>) -> impl Logger {
    InfoLogger::with_request(RequestEntry::new(target, request))
}
pub fn verbose_logger<T>(target: &'static str, request: &Request<T>) -> impl Logger {
    VerboseLogger::with_request(RequestEntry::new(target, request))
}

#[derive(Debug, Clone, Serialize)]
struct RequestEntry {
    id: Option<String>,
    target: &'static str,
}

impl RequestEntry {
    fn new<T>(target: &'static str, request: &Request<T>) -> Self {
        let id = request
            .metadata()
            .get(METADATA_REQUEST_ID)
            .and_then(|id| id.to_str().ok())
            .map(|id| id.into());

        Self { id, target }
    }
}
