use crate::z_details::_common::logger::LogLevel;

use crate::auth::remote::service::data::AuthServiceError;

impl AuthServiceError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidArgument(_) => LogLevel::Error,
            Self::AlreadyExists(_) => LogLevel::Audit,
            Self::Unauthenticated(_) => LogLevel::Audit,
            Self::PermissionDenied(_) => LogLevel::Audit,
            Self::Internal(_) => LogLevel::Error,
            Self::Cancelled(_) => LogLevel::Error,
            Self::InfraError(_) => LogLevel::Error,
        }
    }
}
