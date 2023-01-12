use crate::common::api::logger::infra::{LogFilter, LogLevel};

use crate::common::api::request::data::MetadataError;

impl LogFilter for MetadataError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
