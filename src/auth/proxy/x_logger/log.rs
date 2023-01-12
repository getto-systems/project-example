use crate::common::api::logger::infra::{LogFilter, LogLevel};

use crate::auth::proxy::data::AuthProxyError;

impl LogFilter for AuthProxyError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Unauthenticated(_) => LogLevel::Important,
            Self::InfraError(_) => LogLevel::Error,
            Self::ServiceConnectError(err) => err.log_level(),
            Self::ServiceMetadataError(err) => err.log_level(),
            Self::MessageError(err) => err.log_level(),
        }
    }
}
