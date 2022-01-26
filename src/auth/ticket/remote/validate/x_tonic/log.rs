use crate::z_lib::remote::logger::LogLevel;

use crate::auth::ticket::remote::validate::method::ValidateAuthTokenEvent;

use crate::auth::ticket::remote::validate::data::ValidateAuthTokenError;

impl ValidateAuthTokenEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::ValidateNonce(event) => event.log_level(),
            Self::Success(_) => LogLevel::Audit,
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
        }
    }
}
