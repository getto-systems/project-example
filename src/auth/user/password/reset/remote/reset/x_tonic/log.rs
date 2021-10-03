use crate::z_lib::remote::logger::LogLevel;

use crate::auth::user::password::reset::remote::reset::event::ResetPasswordEvent;

use crate::auth::user::password::reset::remote::{
    kernel::data::ValidateResetTokenError,
    reset::data::{DecodeResetTokenError, NotifyResetPasswordError, ResetPasswordError},
};

impl ResetPasswordEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::ResetNotified(_) => LogLevel::Info,
            Self::Success(_) => LogLevel::Audit,
            Self::InvalidReset(err) => err.log_level(),
            Self::UserNotFound => LogLevel::Error,
            Self::NonceError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
            Self::PasswordHashError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::NotifyError(err) => err.log_level(),
        }
    }
}

impl ResetPasswordError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidLoginId(_) => LogLevel::Error,
            Self::InvalidPassword(_) => LogLevel::Error,
            Self::InvalidResetToken(_) => LogLevel::Error,
            Self::InvalidResetTokenEntry(_) => LogLevel::Audit,
        }
    }
}

impl NotifyResetPasswordError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}

impl DecodeResetTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Expired => LogLevel::Debug,
            Self::Invalid(_) => LogLevel::Audit,
        }
    }
}

impl ValidateResetTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Empty => LogLevel::Error,
        }
    }
}
