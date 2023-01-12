use crate::common::api::logger::infra::{LogFilter, LogLevel};

use crate::auth::user::password::kernel::data::{PasswordHashError, ValidatePasswordError};

impl LogFilter for PasswordHashError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}

impl LogFilter for ValidatePasswordError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Password(err) => err.log_level(),
        }
    }
}
