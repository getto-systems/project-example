use crate::x_outside_feature::_api::logger::LogLevel;

use super::super::event::ValidateAuthTokenEvent;

use super::super::data::{DecodeAuthTokenError, ValidateAuthTokenError};

impl ValidateAuthTokenEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::NonceError(err) => err.log_level(),
            Self::TokenError(err) => err.log_level(),
        }
    }
}

impl ValidateAuthTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::HeaderError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
            Self::PermissionError(err) => err.log_level(),
        }
    }
}

impl DecodeAuthTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Expired => LogLevel::Debug,
            Self::Invalid(_) => LogLevel::Audit,
        }
    }
}
