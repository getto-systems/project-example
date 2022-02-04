use crate::z_lib::api::logger::{LogFilter, LogLevel};

use crate::z_lib::api::repository::data::RepositoryError;

impl LogFilter for RepositoryError {
    fn log_level(&self) -> LogLevel {
        LogLevel::Error
    }
}
