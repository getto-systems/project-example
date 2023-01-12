use crate::auth::user::password::authenticate::proxy::action::AuthenticateWithPasswordProxyState;

use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for AuthenticateWithPasswordProxyState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for AuthenticateWithPasswordProxyState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::ProxyCall(event) => event.log_level(),
        }
    }
}
