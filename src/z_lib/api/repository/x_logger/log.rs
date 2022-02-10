use crate::z_lib::logger::infra::{LogFilter, LogLevel};

use crate::z_lib::repository::data::RepositoryError;

impl LogFilter for RepositoryError {
    fn log_level(&self) -> LogLevel {
        LogLevel::Error
    }
}
