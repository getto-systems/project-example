use crate::z_lib::remote::logger::LogLevel;

use super::super::data::ValidateAuthNonceError;

impl ValidateAuthNonceError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::NonceNotSent => LogLevel::Error,
            Self::MetadataError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
            Self::Conflict => LogLevel::Audit,
        }
    }
}
