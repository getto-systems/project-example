use crate::auth::{
    _common::service::data::AuthServiceError,
    auth_ticket::_common::kernel::data::AuthServiceMetadataError,
};

pub enum LogoutEvent {
    Success,
    MetadataError(AuthServiceMetadataError),
    ServiceError(AuthServiceError),
}

const SUCCESS: &'static str = "logout success";
const ERROR: &'static str = "logout error";

impl std::fmt::Display for LogoutEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "{}", SUCCESS),
            Self::MetadataError(err) => write!(f, "{}: {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
