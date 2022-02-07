use crate::z_lib::logger::{LogFilter, LogLevel};

use crate::z_lib::request::data::MetadataError;

impl LogFilter for MetadataError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(_) => LogLevel::Error,
        }
    }
}
