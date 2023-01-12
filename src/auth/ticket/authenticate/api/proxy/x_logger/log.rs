use crate::auth::ticket::authenticate::proxy::action::AuthenticateWithTokenProxyState;

use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for AuthenticateWithTokenProxyState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for AuthenticateWithTokenProxyState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::AuthenticateWithToken(event) => event.log_level(),
            Self::ProxyCall(event) => event.log_level(),
        }
    }
}
