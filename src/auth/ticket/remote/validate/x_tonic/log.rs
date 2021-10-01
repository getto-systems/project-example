use crate::z_lib::remote::logger::LogLevel;

use crate::auth::ticket::remote::validate::event::ValidateAuthTokenEvent;

use crate::auth::ticket::remote::validate::data::ValidateAuthTokenError;

impl ValidateAuthTokenEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Audit,
            Self::NonceError(err) => err.log_level(),
            Self::TokenError(err) => err.log_level(),
            Self::PermissionError(err) => err.log_level(),
        }
    }
}

impl ValidateAuthTokenError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::TokenNotSent => LogLevel::Info,
            Self::MetadataError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}