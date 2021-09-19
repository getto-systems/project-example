use crate::{
    auth::_common::data::{AuthUserId, ValidateApiTokenError},
    example::_api::service::data::ExampleServiceError,
    z_details::{_api::{message::data::MessageError, }, _common::request::data::MetadataError},
};

pub enum GetOutlineMenuBadgeEvent {
    Authorized(AuthUserId),
    Success(String),
    ValidateApiTokenError(ValidateApiTokenError),
    ServiceError(ExampleServiceError),
    MetadataError(MetadataError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "get outline menu badge success";
const ERROR: &'static str = "get outline menu badge error";

impl std::fmt::Display for GetOutlineMenuBadgeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authorized(user_id) => write!(f, "authorized; {}", user_id),
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::ValidateApiTokenError(err) => write!(f, "{}: {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}: {}", ERROR, err),
            Self::MetadataError(err) => write!(f, "{}: {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
