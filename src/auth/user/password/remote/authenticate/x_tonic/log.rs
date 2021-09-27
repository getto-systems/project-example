use crate::z_details::_common::logger::LogLevel;

use crate::auth::user::password::remote::authenticate::data::AuthenticatePasswordError;

use super::super::event::AuthenticatePasswordEvent;

impl AuthenticatePasswordEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::UserNotFound => LogLevel::Error,
            Self::InvalidPassword(err) => err.log_level(),
            Self::NonceError(err) => err.log_level(),
            Self::PasswordHashError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}

impl AuthenticatePasswordError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidLoginId(_) => LogLevel::Error,
            Self::InvalidPassword(_) => LogLevel::Error,
            Self::PasswordNotFound => LogLevel::Error,
            Self::PasswordNotMatched => LogLevel::Audit,
        }
    }
}
