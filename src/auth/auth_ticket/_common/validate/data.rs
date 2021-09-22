use crate::{
    auth::{
        _common::service::data::AuthServiceError,
        auth_ticket::_common::kernel::data::DecodeAuthTokenError,
    },
    z_details::_common::request::data::MetadataError,
};

pub enum ValidateApiTokenError {
    ServiceError(AuthServiceError),
    MetadataError(MetadataError),
    DecodeError(DecodeAuthTokenError),
}

const ERROR: &'static str = "validate api token error";

impl std::fmt::Display for ValidateApiTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MetadataError(err) => write!(f, "{}; {}", ERROR, err),
            Self::DecodeError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}
