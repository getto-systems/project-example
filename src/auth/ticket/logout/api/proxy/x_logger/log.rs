use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

use crate::auth::ticket::logout::proxy::action::LogoutProxyState;

impl LogMessage for LogoutProxyState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for LogoutProxyState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::AuthenticateWithToken(event) => event.log_level(),
            Self::ProxyCall(event) => event.log_level(),
        }
    }
}
