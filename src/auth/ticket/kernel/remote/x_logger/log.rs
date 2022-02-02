use crate::z_lib::remote::logger::{LogLevel, LogFilter};

use crate::auth::ticket::kernel::remote::data::{DecodeAuthTokenError, ValidateAuthRolesError};

impl LogFilter for DecodeAuthTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Expired => LogLevel::Debug,
            Self::Invalid(_) => LogLevel::Audit,
        }
    }
}

impl LogFilter for ValidateAuthRolesError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::PermissionDenied(_, _) => LogLevel::Audit,
        }
    }
}