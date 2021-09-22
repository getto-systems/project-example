use crate::auth::_common::data::ValidateApiTokenError;

pub enum NotifyUnexpectedErrorEvent {
    Error(String),
    ValidateError(ValidateApiTokenError),
}

impl std::fmt::Display for NotifyUnexpectedErrorEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error(err) => write!(f, "{}", err),
            Self::ValidateError(err) => err.fmt(f),
        }
    }
}
