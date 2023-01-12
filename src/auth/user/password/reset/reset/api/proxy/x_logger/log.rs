use crate::auth::user::password::reset::reset::proxy::action::ResetPasswordProxyState;

use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

impl LogMessage for ResetPasswordProxyState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for ResetPasswordProxyState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::ProxyCall(event) => event.log_level(),
        }
    }
}
