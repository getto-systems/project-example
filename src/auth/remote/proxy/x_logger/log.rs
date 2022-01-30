use crate::z_lib::remote::logger::{LogLevel, LogMessage, LogFilter};

use crate::auth::remote::proxy::action::AuthProxyState;

use crate::auth::remote::proxy::data::AuthProxyError;

impl<R> LogMessage for AuthProxyState<R> {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl<R> LogFilter for AuthProxyState<R> {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Metadata(event) => event.log_level(),
            Self::TryToCall(_) => LogLevel::Info,
            Self::Response(_) => LogLevel::Debug,
            Self::ServiceError(err) => err.log_level(),
        }
    }
}

impl LogFilter for AuthProxyError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::AlreadyExists(_) => LogLevel::Audit,
            Self::Unauthenticated(_) => LogLevel::Audit,
            Self::PermissionDenied(_) => LogLevel::Audit,
            Self::Cancelled(_) => LogLevel::Error,
            Self::InfraError(_) => LogLevel::Error,
            Self::MessageError(err) => err.log_level(),
        }
    }
}
