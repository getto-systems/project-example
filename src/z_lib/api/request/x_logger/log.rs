use crate::z_lib::api::logger::{LogFilter, LogLevel};

use crate::z_lib::api::request::data::MetadataError;

impl LogFilter for MetadataError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
