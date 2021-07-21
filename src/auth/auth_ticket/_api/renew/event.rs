use std::fmt::Display;

use crate::{
    auth::{_api::service::data::ServiceError, auth_ticket::_api::kernel::data::AuthTokenMessage},
    z_details::_api::{message::data::MessageError, request::data::HeaderError},
};

pub enum RenewAuthTicketEvent {
    Success(AuthTokenMessage),
    HeaderError(HeaderError),
    ServiceError(ServiceError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "renew success";
const ERROR: &'static str = "renew error";

impl Display for RenewAuthTicketEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::HeaderError(err) => write!(f, "{}: {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}: {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
