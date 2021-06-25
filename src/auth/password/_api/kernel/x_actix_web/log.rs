use crate::x_outside_feature::_api::logger::LogLevel;

use crate::auth::password::_api::kernel::data::{PasswordHashError, ValidatePasswordError};

impl PasswordHashError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}

impl ValidatePasswordError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Empty => LogLevel::Error,
            Self::TooLong => LogLevel::Error,
        }
    }
}