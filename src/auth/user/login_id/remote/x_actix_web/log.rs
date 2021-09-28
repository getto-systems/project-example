use crate::z_lib::remote::logger::LogLevel;

use super::super::data::ValidateLoginIdError;

impl ValidateLoginIdError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Empty => LogLevel::Error,
            Self::TooLong => LogLevel::Error,
        }
    }
}
