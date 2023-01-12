use crate::auth::user::password::reset::request_token::proxy::action::RequestResetTokenProxyState;

use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for RequestResetTokenProxyState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for RequestResetTokenProxyState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::ProxyCall(event) => event.log_level(),
        }
    }
}
