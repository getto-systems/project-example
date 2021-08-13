use std::fmt::Display;

use crate::{
    auth::{
        _api::service::data::ServiceError,
        password::reset::_api::request_token::data::RequestResetTokenResult,
    },
    z_details::_api::{message::data::MessageError, request::data::HeaderError},
};

pub enum RequestResetTokenEvent {
    Result(RequestResetTokenResult),
    HeaderError(HeaderError),
    ServiceError(ServiceError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "request reset token";
const ERROR: &'static str = "request reset token error";

impl Display for RequestResetTokenEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Result(message) => write!(f, "{}", message),
            Self::HeaderError(err) => write!(f, "{}; {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}; {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}; {}", ERROR, err),
        }
    }
}

impl Display for RequestResetTokenResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::InvalidRequest(_) => write!(f, "{}; invalid request", ERROR),
        }
    }
}
