use crate::z_lib::logger::infra::{LogFilter, LogLevel};

use crate::auth::ticket::kernel::data::{DecodeAuthTokenError, PermissionError};

impl LogFilter for DecodeAuthTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Expired => LogLevel::Debug,
            Self::Invalid(_) => LogLevel::Audit,
        }
    }
}

impl LogFilter for PermissionError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::PermissionDenied(_, _) => LogLevel::Audit,
        }
    }
}
