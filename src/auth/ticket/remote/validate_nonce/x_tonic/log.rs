use crate::z_lib::remote::logger::LogLevel;

use super::super::method::ValidateAuthNonceEvent;

impl ValidateAuthNonceEvent {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::NonceExpiresCalculated(_) => LogLevel::Debug,
            Self::Success => LogLevel::Info,
            Self::NonceNotSent => LogLevel::Error,
            Self::MetadataError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
            Self::Conflict => LogLevel::Audit,
        }
    }
}
