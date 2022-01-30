use crate::z_lib::remote::logger::{LogFilter, LogLevel};

use crate::auth::user::password::remote::kernel::data::{PasswordHashError, ValidatePasswordError};

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
            Self::Empty => LogLevel::Error,
            Self::TooLong => LogLevel::Error,
        }
    }
}
