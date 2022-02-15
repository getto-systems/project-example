use crate::auth::ticket::validate::action::ValidateApiTokenState;

use crate::auth::ticket::validate::method::{
    CheckPermissionEvent, ValidateAuthMetadataEvent, ValidateAuthNonceEvent, ValidateAuthTokenEvent,
};

use crate::z_lib::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for ValidateApiTokenState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for ValidateApiTokenState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Validate(event) => event.log_level(),
            Self::PermissionError(err) => err.log_level(),
            Self::Success(_) => LogLevel::Info,
        }
    }
}

impl LogFilter for ValidateAuthTokenEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::ValidateNonce(event) => event.log_level(),
            Self::Success(_) => LogLevel::Audit,
            Self::TokenNotSent => LogLevel::Info,
            Self::MetadataError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
        }
    }
}

impl LogFilter for CheckPermissionEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Info,
            Self::MetadataError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::ServiceError(err) => err.log_level(),
        }
    }
}

impl LogFilter for ValidateAuthMetadataEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Debug,
            Self::MetadataError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
        }
    }
}

impl LogFilter for ValidateAuthNonceEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::NonceExpiresCalculated(_) => LogLevel::Debug,
            Self::Success => LogLevel::Debug,
            Self::NonceNotSent => LogLevel::Error,
            Self::MetadataError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
            Self::Conflict => LogLevel::Audit,
        }
    }
}
