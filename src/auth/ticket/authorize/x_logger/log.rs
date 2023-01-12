use crate::auth::ticket::authorize::action::{
    ClarifyAuthorizeTokenEvent, ClarifyAuthorizeTokenState,
};

use crate::auth::ticket::authorize::{method::AuthorizeWithTokenEvent, proxy::AuthorizeEvent};

use crate::common::api::logger::infra::{LogFilter, LogLevel, LogMessage};

use crate::auth::ticket::authorize::data::ValidateAuthorizeFieldsError;

impl LogMessage for ClarifyAuthorizeTokenState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for ClarifyAuthorizeTokenState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::AuthorizeWithToken(event) => event.log_level(),
            Self::ClarifyAuthorizeToken(event) => event.log_level(),
        }
    }
}

impl LogFilter for AuthorizeWithTokenEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success => LogLevel::Important,
            Self::Invalid(err) => err.log_level(),
            Self::DecodeError(err) => err.log_level(),
            Self::PermissionError(err) => err.log_level(),
        }
    }
}

impl LogFilter for ClarifyAuthorizeTokenEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::TicketNotFound => LogLevel::Error,
            Self::TicketHasExpired => LogLevel::Error,
            Self::UserNotFound => LogLevel::Error,
            Self::Success(_) => LogLevel::Important,
            Self::RepositoryError(err) => err.log_level(),
            Self::PermissionError(err) => err.log_level(),
        }
    }
}

impl LogFilter for AuthorizeEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Invalid(err) => err.log_level(),
            Self::ProxyCall(event) => event.log_level(),
        }
    }
}

impl LogFilter for ValidateAuthorizeFieldsError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Token(err) => err.log_level(),
            Self::Required(err) => err.log_level(),
        }
    }
}
