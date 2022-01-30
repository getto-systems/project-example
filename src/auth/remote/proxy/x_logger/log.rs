use crate::z_lib::remote::logger::{LogLevel, LogMessage, LogFilter};

use crate::auth::remote::proxy::action::AuthProxyState;

use crate::auth::remote::proxy::data::AuthProxyError;

impl<R, E: std::fmt::Display> LogMessage for AuthProxyState<R, E> {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl<R, E: LogFilter> LogFilter for AuthProxyState<R, E> {
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
