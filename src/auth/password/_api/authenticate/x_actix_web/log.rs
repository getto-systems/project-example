use crate::z_details::_api::logger::LogLevel;

use crate::auth::password::_api::authenticate::data::AuthenticatePasswordResponse;

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
            Self::MessageError(err) => err.log_level(),
            Self::ValidateLoginIdError(err) => err.log_level(),
            Self::ValidatePasswordError(err) => err.log_level(),
        }
    }
}

impl AuthenticatePasswordResponse {
    pub const fn log_level(&self) -> LogLevel {
        LogLevel::Audit
    }
}
