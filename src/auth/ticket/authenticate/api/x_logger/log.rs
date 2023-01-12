use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

use crate::auth::ticket::authenticate::action::AuthenticateWithTokenState;

use crate::auth::ticket::authenticate::method::AuthenticateWithTokenEvent;

impl LogMessage for AuthenticateWithTokenState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for AuthenticateWithTokenState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::AuthenticateWithToken(event) => event.log_level(),
            Self::Encode(event) => event.log_level(),
        }
    }
}

impl LogFilter for AuthenticateWithTokenEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Important,
            Self::Invalid(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
        }
    }
}
