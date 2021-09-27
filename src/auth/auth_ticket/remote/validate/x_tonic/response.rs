use tonic::{Response, Status};

use crate::auth::auth_ticket::_common::y_protobuf::service::{
    LogoutResponsePb, RenewAuthTicketResponsePb, ValidateApiTokenResponsePb,
};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::auth::auth_ticket::remote::validate::event::ValidateAuthTokenEvent;

use crate::auth::auth_ticket::remote::validate::data::ValidateAuthTokenError;

impl RespondTo<LogoutResponsePb> for ValidateAuthTokenEvent {
    fn respond_to(self) -> Result<Response<LogoutResponsePb>, Status> {
        match self {
            Self::Success(_) => Err(Status::cancelled("logout cancelled")),
            Self::NonceError(err) => err.respond_to(),
            Self::TokenError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}
impl RespondTo<RenewAuthTicketResponsePb> for ValidateAuthTokenEvent {
    fn respond_to(self) -> Result<Response<RenewAuthTicketResponsePb>, Status> {
        match self {
            Self::Success(_) => Err(Status::cancelled("renew auth ticket cancelled")),
            Self::NonceError(err) => err.respond_to(),
            Self::TokenError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}
impl RespondTo<ValidateApiTokenResponsePb> for ValidateAuthTokenEvent {
    fn respond_to(self) -> Result<Response<ValidateApiTokenResponsePb>, Status> {
        match self {
            Self::Success(_) => Err(Status::cancelled("validate api token cancelled")),
            Self::NonceError(err) => err.respond_to(),
            Self::TokenError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}

impl<T> RespondTo<T> for ValidateAuthTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::TokenNotSent => Err(Status::unauthenticated(format!("{}", self))),
            Self::MetadataError(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
