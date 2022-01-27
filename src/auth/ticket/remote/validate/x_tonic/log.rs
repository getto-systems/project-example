use crate::z_lib::remote::logger::LogLevel;

use crate::auth::ticket::remote::validate::method::{
    ValidateApiTokenEvent, ValidateAuthTokenEvent,
};

impl ValidateAuthTokenEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::ValidateNonce(event) => event.log_level(),
            Self::Success(_) => LogLevel::Audit,
            Self::TokenNotSent => LogLevel::Info,
            Self::MetadataError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::PermissionError(err) => err.log_level(),
        }
    }
}

impl ValidateApiTokenEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Info,
            Self::MetadataError(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::ServiceError(err) => err.log_level(),
        }
    }
}
