use crate::{
    auth::_common::data::AuthServiceMetadataError,
    example::_api::service::data::ExampleServiceError,
    z_details::_api::message::data::MessageError,
};

pub enum GetOutlineMenuBadgeEvent {
    Success(String),
    MetadataError(AuthServiceMetadataError),
    ServiceError(ExampleServiceError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "get outline menu badge success";
const ERROR: &'static str = "get outline menu badge error";

impl std::fmt::Display for GetOutlineMenuBadgeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::MetadataError(err) => write!(f, "{}: {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}: {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
