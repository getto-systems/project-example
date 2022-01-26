use crate::z_lib::remote::logger::{LogLevel, LogMessage};

use super::super::action::{RequestResetTokenEvent, RequestResetTokenState};

use crate::auth::user::password::reset::remote::request_token::data::{
    EncodeResetTokenError, NotifyResetTokenError, RequestResetTokenError,
};

impl LogMessage for &RequestResetTokenState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl RequestResetTokenState {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::ValidateNonce(event) => event.log_level(),
            Self::RequestToken(event) => event.log_level(),
        }
    }
}

impl RequestResetTokenEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::TokenExpiresCalculated(_) => LogLevel::Info,
            Self::TokenNotified(_) => LogLevel::Info,
            Self::Success => LogLevel::Audit,
            Self::InvalidRequest(err) => err.log_level(),
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
