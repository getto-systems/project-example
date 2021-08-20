use crate::{
    auth::_api::{
        common::data::{AuthUserId, ValidateApiTokenError},
        service::data::AuthServiceError,
    },
    z_details::_api::request::data::HeaderError,
};

pub enum NotifyUnexpectedErrorEvent {
    Authorized(AuthUserId),
    Notice(String),
    ValidateApiTokenError(ValidateApiTokenError),
    ServiceError(AuthServiceError),
    HeaderError(HeaderError),
}

const ERROR: &'static str = "notify unexpected error error";

impl std::fmt::Display for NotifyUnexpectedErrorEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorized(user_id) => write!(f, "authorized; {}", user_id),
            Self::Notice(err) => write!(f, "{}", err),
            Self::ValidateApiTokenError(err) => write!(f, "{}: {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}: {}", ERROR, err),
            Self::HeaderError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
