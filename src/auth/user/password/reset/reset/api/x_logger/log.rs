use super::super::action::{ResetPasswordEvent, ResetPasswordState};

use crate::auth::user::password::reset::reset::data::ValidateResetPasswordFieldsError;
use crate::z_lib::logger::infra::{LogFilter, LogLevel, LogMessage};

use crate::auth::user::password::reset::{
    kernel::data::ValidateResetTokenError,
    reset::data::{DecodeResetTokenError, NotifyResetPasswordError},
};

impl LogMessage for ResetPasswordState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for ResetPasswordState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::ValidateNonce(event) => event.log_level(),
            Self::Reset(event) => event.log_level(),
            Self::Issue(event) => event.log_level(),
            Self::Encode(event) => event.log_level(),
        }
    }
}

impl LogFilter for ResetPasswordEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::ResetNotified(_) => LogLevel::Info,
            Self::Success(_) => LogLevel::Audit,
            Self::Invalid(err) => err.log_level(),
            Self::NotFound => LogLevel::Error,
            Self::ResetTokenExpired => LogLevel::Audit,
            Self::LoginIdNotMatched => LogLevel::Audit,
            Self::AlreadyReset => LogLevel::Audit,
            Self::RepositoryError(err) => err.log_level(),
            Self::PasswordHashError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::NotifyError(err) => err.log_level(),
        }
    }
}

impl LogFilter for ValidateResetPasswordFieldsError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidResetToken(_) => LogLevel::Error,
            Self::InvalidLoginId(_) => LogLevel::Error,
            Self::InvalidNewPassword(_) => LogLevel::Error,
        }
    }
}

impl LogFilter for NotifyResetPasswordError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}

impl LogFilter for DecodeResetTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Expired => LogLevel::Error,
            Self::Invalid(_) => LogLevel::Audit,
        }
    }
}

impl LogFilter for ValidateResetTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Text(err) => err.log_level(),
        }
    }
}
