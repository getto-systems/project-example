use super::super::action::{RequestResetTokenEvent, RequestResetTokenState};

use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

use crate::auth::user::password::reset::request_token::data::{
    EncodeResetTokenError, NotifyResetTokenError,
};

impl LogMessage for RequestResetTokenState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for RequestResetTokenState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::RequestToken(event) => event.log_level(),
        }
    }
}

impl LogFilter for RequestResetTokenEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::TokenExpiresCalculated(_) => LogLevel::Info,
            Self::TokenNotified(_) => LogLevel::Info,
            Self::Success => LogLevel::Important,
            Self::Invalid(_) => LogLevel::Error,
            Self::NotFound => LogLevel::Error,
            Self::RepositoryError(err) => err.log_level(),
            Self::EncodeError(err) => err.log_level(),
            Self::NotifyError(err) => err.log_level(),
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
            Self::NoDestination => LogLevel::Info,
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}
