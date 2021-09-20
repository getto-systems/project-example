use crate::auth::{
    _common::service::data::AuthServiceError,
    auth_ticket::_common::kernel::data::AuthMetadataError,
};

pub enum ValidateApiTokenError {
    ServiceError(AuthServiceError),
    MetadataError(AuthMetadataError),
}

const ERROR: &'static str = "validate api token error";

impl std::fmt::Display for ValidateApiTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MetadataError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}
