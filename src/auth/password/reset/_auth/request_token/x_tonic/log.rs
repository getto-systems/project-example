use crate::z_details::_common::logger::LogLevel;

use crate::auth::password::reset::_auth::request_token::event::RequestResetTokenEvent;

use crate::auth::password::reset::_auth::request_token::data::{
    EncodeResetTokenError, NotifyResetTokenError, RequestResetTokenError,
};

impl RequestResetTokenEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::TokenExpiresCalculated(_) => LogLevel::Info,
            Self::TokenNotified(_) => LogLevel::Info,
            Self::Success => LogLevel::Audit,
            Self::InvalidRequest(err) => err.log_level(),
            Self::NonceError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
            Self::EncodeError(err) => err.log_level(),
            Self::NotifyError(err) => err.log_level(),
        }
    }
}

impl RequestResetTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidLoginId(_) => LogLevel::Error,
            Self::UserNotFound => LogLevel::Audit,
            Self::DestinationNotFound => LogLevel::Audit,
        }
    }
}

impl EncodeResetTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}

impl NotifyResetTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}
