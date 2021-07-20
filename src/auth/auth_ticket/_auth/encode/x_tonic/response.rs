use tonic::{Response, Status};

use crate::auth::{
    auth_ticket::_common::y_protobuf::service::RenewAuthTicketResponsePb,
    password::_common::y_protobuf::service::AuthenticatePasswordResponsePb,
};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::auth::auth_ticket::_auth::encode::event::EncodeAuthTicketEvent;

use crate::auth::auth_ticket::_auth::encode::data::EncodeAuthTokenError;

impl RespondTo<RenewAuthTicketResponsePb> for EncodeAuthTicketEvent {
    fn respond_to(self) -> Result<Response<RenewAuthTicketResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => token_expires_calculated(),
            Self::Success(response) => response.respond_to(),
            Self::TicketNotFound => ticket_not_found(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

impl RespondTo<AuthenticatePasswordResponsePb> for EncodeAuthTicketEvent {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => token_expires_calculated(),
            Self::Success(response) => response.respond_to(),
            Self::TicketNotFound => ticket_not_found(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

fn token_expires_calculated<T>() -> Result<Response<T>, Status> {
    Err(Status::cancelled("renew auth ticket cancelled"))
}
fn ticket_not_found<T>() -> Result<Response<T>, Status> {
    Err(Status::unauthenticated("ticket not found"))
}

impl<T> RespondTo<T> for EncodeAuthTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("{}", self)))
    }
}
