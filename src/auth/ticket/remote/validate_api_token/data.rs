use crate::{
    auth::{
        remote::service::data::AuthServiceError,
        ticket::remote::kernel::data::DecodeAuthTokenError,
    },
    z_lib::remote::request::data::MetadataError,
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
