use std::fmt::Display;

use crate::{
    auth::_api::service::data::AuthServiceError, z_details::_api::request::data::HeaderError,
};

pub enum ValidateApiTokenEvent {
    Success,
    ServiceError(AuthServiceError),
    HeaderError(HeaderError),
}

const SUCCESS: &'static str = "validate success";
const ERROR: &'static str = "validate error";

impl Display for ValidateApiTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "{}", SUCCESS),
            Self::ServiceError(err) => write!(f, "{}: {}", ERROR, err),
            Self::HeaderError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
