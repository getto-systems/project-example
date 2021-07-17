use crate::z_details::_api::logger::LogLevel;

use crate::auth::password::reset::_api::request_token::event::RequestResetTokenEvent;

use crate::auth::password::reset::_api::request_token::data::{
    EncodeResetTokenError, NotifyResetTokenError,
};

impl RequestResetTokenEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::TokenExpiresCalculated(_) => LogLevel::Info,
            Self::TokenNotified(_) => LogLevel::Info,
            Self::Success(_) => LogLevel::Audit,
            Self::InvalidReset(_) => LogLevel::Error,
            Self::NonceError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
            Self::MessageError(err) => err.log_level(),
            Self::EncodeError(err) => err.log_level(),
            Self::NotifyError(err) => err.log_level(),
            Self::ValidateLoginIdError(err) => err.log_level(),
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
