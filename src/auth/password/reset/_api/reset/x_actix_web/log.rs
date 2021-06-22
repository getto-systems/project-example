use crate::{
    auth::password::reset::_api::{
        kernel::data::ValidateResetTokenError, reset::data::DecodeResetTokenError,
    },
    x_outside_feature::_api::logger::LogLevel,
};

use super::super::event::ResetPasswordEvent;

impl ResetPasswordEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::InvalidReset(_) => LogLevel::Error,
            Self::UserNotFound => LogLevel::Error,
            Self::NonceError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
            Self::PasswordHashError(err) => err.log_level(),
            Self::MessageError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::ValidateLoginIdError(err) => err.log_level(),
            Self::ValidatePasswordError(err) => err.log_level(),
            Self::ValidateResetTokenError(err) => err.log_level(),
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
