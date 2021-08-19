use crate::{
    auth::{
        _api::service::data::AuthServiceError, auth_ticket::_api::kernel::data::AuthTokenResponse,
    },
    z_details::_api::{message::data::MessageError, request::data::HeaderError},
};

pub enum RenewAuthTicketEvent {
    Success(AuthTokenResponse),
    HeaderError(HeaderError),
    ServiceError(AuthServiceError),
    MessageError(MessageError),
}

const SUCCESS: &'static str = "renew success";
const ERROR: &'static str = "renew error";

impl std::fmt::Display for RenewAuthTicketEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success(_) => write!(f, "{}", SUCCESS),
            Self::HeaderError(err) => write!(f, "{}: {}", ERROR, err),
            Self::ServiceError(err) => write!(f, "{}: {}", ERROR, err),
            Self::MessageError(err) => write!(f, "{}: {}", ERROR, err),
        }
    }
}
