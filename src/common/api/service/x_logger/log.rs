use crate::common::api::logger::infra::{LogFilter, LogLevel};

use crate::common::api::service::data::{ServiceConnectError, ServiceMetadataError};

impl LogFilter for ServiceConnectError {
    fn log_level(&self) -> LogLevel {
        LogLevel::Error
    }
}

impl LogFilter for ServiceMetadataError {
    fn log_level(&self) -> LogLevel {
        LogLevel::Error
    }
}
