use crate::common::api::logger::infra::{LogFilter, LogLevel};

use crate::auth::ticket::kernel::data::{
    AuthPermissionError, DecodeAuthenticateTokenError, DecodeAuthorizeTokenError,
    ValidateAuthPermissionError, ValidateAuthenticateTokenError, ValidateAuthorizeTokenError,
};

impl LogFilter for ValidateAuthenticateTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::NotFound => LogLevel::Error,
            Self::MetadataError(_) => LogLevel::Error,
        }
    }
}

impl LogFilter for DecodeAuthenticateTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Expired => LogLevel::Debug,
            Self::Invalid(_) => LogLevel::Important,
        }
    }
}

impl LogFilter for ValidateAuthorizeTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::NotFound => LogLevel::Error,
            Self::MetadataError(_) => LogLevel::Error,
        }
    }
}

impl LogFilter for DecodeAuthorizeTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Expired => LogLevel::Debug,
            Self::Invalid(_) => LogLevel::Important,
        }
    }
}

impl LogFilter for ValidateAuthPermissionError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid => LogLevel::Error,
        }
    }
}

impl LogFilter for AuthPermissionError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::PermissionDenied(_, _) => LogLevel::Important,
        }
    }
}
