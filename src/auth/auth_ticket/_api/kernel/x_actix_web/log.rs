use crate::z_details::_api::logger::LogLevel;

use super::super::data::{ValidateAuthNonceError, ValidateAuthRolesError};

impl ValidateAuthNonceError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::HeaderError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
            Self::Conflict => LogLevel::Audit,
        }
    }
}

impl ValidateAuthRolesError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::PermissionDenied(_, _) => LogLevel::Audit,
        }
    }
}
