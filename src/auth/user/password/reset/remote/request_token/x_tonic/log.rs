use crate::z_lib::remote::logger::{LogFilter, LogLevel, LogMessage};

use super::super::action::{RequestResetTokenEvent, RequestResetTokenState};

use crate::auth::user::password::reset::remote::request_token::data::{
    EncodeResetTokenError, NotifyResetTokenError, RequestResetTokenError,
};

impl LogMessage for RequestResetTokenState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for RequestResetTokenState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::ValidateNonce(event) => event.log_level(),
            Self::RequestToken(event) => event.log_level(),
        }
    }
}

impl LogFilter for RequestResetTokenEvent {
    fn log_level(&self) -> LogLevel {
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

impl LogFilter for RequestResetTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidLoginId(_) => LogLevel::Error,
            Self::UserNotFound => LogLevel::Audit,
            Self::DestinationNotFound => LogLevel::Audit,
        }
    }
}

impl LogFilter for EncodeResetTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}

impl LogFilter for NotifyResetTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}
