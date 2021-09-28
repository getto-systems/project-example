use serde::Serialize;
use tonic::metadata::MetadataMap;

use crate::x_outside_feature::remote::common::metadata::METADATA_REQUEST_ID;

use crate::z_details::_common::logger::{InfoLogger, Logger, QuietLogger, VerboseLogger};

pub fn app_logger(target: &'static str, metadata: &MetadataMap) -> impl Logger {
    // アプリケーション全体で使用するデフォルトの logger を返す
    // 個別のアクションでレベルを指定したい時はそれぞれ個別のやつを呼び出す
    verbose_logger(target, metadata)
}
pub fn quiet_logger(target: &'static str, metadata: &MetadataMap) -> impl Logger {
    QuietLogger::with_request(RequestEntry::new(target, metadata))
}
pub fn info_logger(target: &'static str, metadata: &MetadataMap) -> impl Logger {
    InfoLogger::with_request(RequestEntry::new(target, metadata))
}
pub fn verbose_logger(target: &'static str, metadata: &MetadataMap) -> impl Logger {
    VerboseLogger::with_request(RequestEntry::new(target, metadata))
}

#[derive(Debug, Clone, Serialize)]
struct RequestEntry {
    id: Option<String>,
    target: &'static str,
}

impl RequestEntry {
    fn new(target: &'static str, metadata: &MetadataMap) -> Self {
        let id = metadata
            .get(METADATA_REQUEST_ID)
            .and_then(|id| id.to_str().ok())
            .map(|id| id.into());

        Self { id, target }
    }
}
