use crate::{
    auth::_common::service::data::AuthServiceError,
    z_details::_common::request::data::MetadataError,
};

pub enum LogoutEvent {
    Success,
    ServiceError(AuthServiceError),
    MetadataError(MetadataError),
}

const SUCCESS: &'static str = "logout success";
const ERROR: &'static str = "logout error";

impl std::fmt::Display for LogoutEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "{}", SUCCESS),
            Self::ServiceError(err) => write!(f, "{}: {}", ERROR, err),
            Self::MetadataError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
