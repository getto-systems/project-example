use tonic::{Response, Status};

use crate::auth::ticket::authorize::y_protobuf::service::ClarifyAuthorizeTokenResponsePb;

use crate::common::api::response::tonic::ServiceResponder;

use crate::auth::ticket::authorize::action::{
    ClarifyAuthorizeTokenEvent, ClarifyAuthorizeTokenState,
};

use crate::auth::ticket::authorize::{method::AuthorizeWithTokenEvent, proxy::AuthorizeEvent};

use crate::auth::ticket::authorize::data::ValidateAuthorizeFieldsError;

impl ServiceResponder<ClarifyAuthorizeTokenResponsePb> for ClarifyAuthorizeTokenState {
    fn respond_to(self) -> Result<Response<ClarifyAuthorizeTokenResponsePb>, Status> {
        match self {
            Self::AuthorizeWithToken(event) => event.respond_to(),
            Self::ClarifyAuthorizeToken(event) => event.respond_to(),
        }
    }
}

impl<R> ServiceResponder<R> for AuthorizeWithTokenEvent {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::Success => Err(Status::cancelled(
                "cancelled at authorize with token succeeded",
            )),
            Self::Invalid(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<ClarifyAuthorizeTokenResponsePb> for ClarifyAuthorizeTokenEvent {
    fn respond_to(self) -> Result<Response<ClarifyAuthorizeTokenResponsePb>, Status> {
        match self {
            Self::TicketNotFound => Err(Status::internal("ticket not found")),
            Self::TicketHasExpired => Err(Status::internal("ticket has expired")),
            Self::UserNotFound => Err(Status::internal("user not found")),
            Self::Success(ticket) => Ok(Response::new(ClarifyAuthorizeTokenResponsePb {
                user_id: ticket.attrs.user_id.extract(),
            })),
            Self::RepositoryError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}

impl<R> ServiceResponder<R> for AuthorizeEvent {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::Invalid(err) => err.respond_to(),
            Self::ProxyCall(event) => event.respond_to(),
        }
    }
}

impl<R> ServiceResponder<R> for ValidateAuthorizeFieldsError {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::Token(err) => err.respond_to(),
            Self::Required(err) => err.respond_to(),
        }
    }
}
