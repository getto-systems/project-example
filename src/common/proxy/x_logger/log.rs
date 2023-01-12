use crate::common::proxy::action::CoreProxyState;

use crate::common::proxy::event::ProxyCallEvent;

use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for CoreProxyState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for CoreProxyState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::AuthorizeWithToken(event) => event.log_level(),
            Self::ProxyCall(event) => event.log_level(),
        }
    }
}

impl<R, E> LogFilter for ProxyCallEvent<R, E> {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::TryToCall(_) => LogLevel::Info,
            Self::Response(_) => LogLevel::Debug,
            Self::ServiceError(_) => LogLevel::Error,
        }
    }
}
