use super::super::action::{AuthenticateWithPasswordEvent, AuthenticateWithPasswordState};

use crate::{
    auth::user::password::authenticate::data::ValidateAuthenticateWithPasswordFieldsError,
    common::api::logger::infra::{LogFilter, LogLevel, LogMessage},
};

impl LogMessage for AuthenticateWithPasswordState {
    fn log_message(&self) -> String {
        format!("{}", self)
    }
}

impl LogFilter for AuthenticateWithPasswordState {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::AuthenticateWithPassword(event) => event.log_level(),
            Self::Issue(event) => event.log_level(),
            Self::Encode(event) => event.log_level(),
        }
    }
}

impl LogFilter for AuthenticateWithPasswordEvent {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::Success(_) => LogLevel::Important,
            Self::Invalid(err) => err.log_level(),
            Self::NotFound => LogLevel::Error,
            Self::PasswordNotMatched => LogLevel::Important,
            Self::PasswordHashError(err) => err.log_level(),
            Self::RepositoryError(err) => err.log_level(),
        }
    }
}

impl LogFilter for ValidateAuthenticateWithPasswordFieldsError {
    fn log_level(&self) -> LogLevel {
        match self {
            Self::InvalidLoginId(err) => err.log_level(),
            Self::InvalidPassword(err) => err.log_level(),
        }
    }
}
