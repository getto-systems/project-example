use std::fmt::Display;

use crate::{
    auth::{
        _api::service::data::ServiceError, password::reset::_api::reset::data::ResetPasswordMessage,
    },
    z_details::_api::{message::data::MessageError, request::data::HeaderError},
};

pub enum ResetPasswordEvent {
    Result(ResetPasswordMessage),
    HeaderError(HeaderError),
    ServiceError(ServiceError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "reset password";
const ERROR: &'static str = "reset password error";

impl Display for ResetPasswordEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Result(message) => write!(f, "{}", message),
            Self::HeaderError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl Display for ResetPasswordMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::InvalidReset(_) => write!(f, "{}; invalid reset", ERROR),
            Self::AlreadyReset(_) => write!(f, "{}; already reset", ERROR),
        }
    }
}
