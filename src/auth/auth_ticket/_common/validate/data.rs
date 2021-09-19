use crate::{
    auth::_common::service::data::AuthServiceError, z_details::_common::request::data::MetadataError,
};

pub enum ValidateApiTokenError {
    ServiceError(AuthServiceError),
    MetadataError(MetadataError),
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
