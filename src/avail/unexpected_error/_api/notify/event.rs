use crate::{
    auth::_common::data::{AuthUserId, ValidateApiTokenError},
    z_details::_common::request::data::MetadataError,
};

pub enum NotifyUnexpectedErrorEvent {
    Authorized(AuthUserId),
    Notice(String),
    ValidateApiTokenError(ValidateApiTokenError),
    MetadataError(MetadataError),
}

const ERROR: &'static str = "notify unexpected error error";

impl std::fmt::Display for NotifyUnexpectedErrorEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorized(user_id) => write!(f, "authorized; {}", user_id),
            Self::Notice(err) => write!(f, "{}", err),
            Self::ValidateApiTokenError(err) => write!(f, "{}: {}", ERROR, err),
            Self::MetadataError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
