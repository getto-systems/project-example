use crate::{
    auth::_api::service::data::AuthServiceError, z_details::_api::request::data::HeaderError,
};

pub enum ValidateApiTokenError {
    ServiceError(AuthServiceError),
    HeaderError(HeaderError),
}

const ERROR: &'static str = "validate api token error";

impl std::fmt::Display for ValidateApiTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::HeaderError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}
