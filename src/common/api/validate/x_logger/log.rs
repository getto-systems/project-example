use crate::common::api::logger::infra::{LogFilter, LogLevel};

use crate::common::api::validate::data::ValidateTextError;

impl LogFilter for ValidateTextError {
    fn log_level(&self) -> LogLevel {
        LogLevel::Error
    }
}
