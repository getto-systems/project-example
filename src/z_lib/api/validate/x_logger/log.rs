use crate::z_lib::logger::infra::{LogFilter, LogLevel};

use crate::z_lib::validate::data::ValidateTextError;

impl LogFilter for ValidateTextError {
    fn log_level(&self) -> LogLevel {
        LogLevel::Error
    }
}
