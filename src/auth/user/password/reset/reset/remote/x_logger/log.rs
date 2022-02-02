use crate::z_lib::remote::logger::{LogFilter, LogLevel, LogMessage};

use super::super::action::{ResetPasswordEvent, ResetPasswordState};

use crate::auth::user::password::reset::{
    remote::kernel::data::ValidateResetTokenError,
    reset::remote::data::{DecodeResetTokenError, NotifyResetPasswordError, ResetPasswordError},
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
            Self::InvalidReset(err) => err.log_level(),
            Self::UserNotFound => LogLevel::Error,
            Self::RepositoryError(err) => err.log_level(),
            Self::PasswordHashError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::NotifyError(err) => err.log_level(),
        }
    }
}

impl LogFilter for ResetPasswordError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidLoginId(_) => LogLevel::Error,
            Self::InvalidPassword(_) => LogLevel::Error,
            Self::InvalidResetToken(_) => LogLevel::Error,
            Self::InvalidResetTokenEntry(_) => LogLevel::Audit,
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
            Self::Expired => LogLevel::Debug,
            Self::Invalid(_) => LogLevel::Audit,
        }
    }
}

impl LogFilter for ValidateResetTokenError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Empty => LogLevel::Error,
        }
    }
}
