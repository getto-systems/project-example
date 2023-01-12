use crate::common::api::logger::infra::{LogFilter, LogLevel};

use crate::common::api::repository::data::RepositoryError;

impl LogFilter for RepositoryError {
    fn log_level(&self) -> LogLevel {
        LogLevel::Error
    }
}
