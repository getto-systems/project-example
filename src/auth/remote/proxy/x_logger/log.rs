use crate::z_lib::remote::logger::{LogLevel, LogMessage};

use crate::auth::remote::proxy::action::AuthProxyState;

use crate::auth::remote::proxy::data::AuthProxyError;

impl<T> LogMessage for &AuthProxyState<T> {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl<T> AuthProxyState<T> {
    pub fn log_level(&self) -> LogLevel {
        match self {
            Self::Metadata(event) => event.log_level(),
            Self::TryToCall(_) => LogLevel::Info,
            Self::Response(_) => LogLevel::Debug,
            Self::ServiceError(err) => err.log_level(),
        }
    }
}

impl AuthProxyError {
    pub const fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidArgument(_) => LogLevel::Error,
            Self::AlreadyExists(_) => LogLevel::Audit,
            Self::Unauthenticated(_) => LogLevel::Audit,
            Self::PermissionDenied(_) => LogLevel::Audit,
            Self::Internal(_) => LogLevel::Error,
            Self::Cancelled(_) => LogLevel::Error,
            Self::InfraError(_) => LogLevel::Error,
            Self::MessageError(err) => err.log_level(),
        }
    }
}
