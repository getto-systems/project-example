use crate::{
    auth::{
        _common::service::data::AuthServiceError,
        auth_ticket::_common::kernel::data::AuthServiceMetadataError,
        password::_api::change::data::ChangePasswordResult,
    },
    z_details::_api::message::data::MessageError,
};

pub enum ChangePasswordEvent {
    Result(ChangePasswordResult),
    MetadataError(AuthServiceMetadataError),
    ServiceError(AuthServiceError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "change password";
const ERROR: &'static str = "change password error";

impl std::fmt::Display for ChangePasswordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Result(result) => result.fmt(f),
            Self::MetadataError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl std::fmt::Display for ChangePasswordResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::InvalidPassword(_) => write!(f, "{}; invalid password", ERROR),
        }
    }
}
