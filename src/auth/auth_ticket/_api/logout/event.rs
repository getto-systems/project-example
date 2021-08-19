use crate::{
    auth::_api::service::data::AuthServiceError, z_details::_api::request::data::HeaderError,
};

pub enum LogoutEvent {
    Success,
    ServiceError(AuthServiceError),
    HeaderError(HeaderError),
}

const SUCCESS: &'static str = "logout success";
const ERROR: &'static str = "logout error";

impl std::fmt::Display for LogoutEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "{}", SUCCESS),
            Self::ServiceError(err) => write!(f, "{}: {}", ERROR, err),
            Self::HeaderError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
