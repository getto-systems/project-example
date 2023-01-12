use tonic::{Response, Status};

use crate::auth::ticket::authenticate::y_protobuf::service::AuthenticateWithTokenResponsePb;

use crate::common::api::response::tonic::ServiceResponder;

use crate::auth::ticket::authenticate::action::AuthenticateWithTokenState;

use crate::auth::ticket::{
    authenticate::method::AuthenticateWithTokenEvent, encode::method::EncodeAuthTokenEvent,
};

use crate::auth::ticket::kernel::data::{AuthPermissionGranted, AuthToken};

impl ServiceResponder<AuthenticateWithTokenResponsePb> for AuthenticateWithTokenState {
    fn respond_to(self) -> Result<Response<AuthenticateWithTokenResponsePb>, Status> {
        match self {
            Self::AuthenticateWithToken(event) => event.respond_to(),
            Self::Encode(event) => event.respond_to(),
        }
    }
}

impl<R> ServiceResponder<R> for AuthenticateWithTokenEvent {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::Success(_) => Err(Status::cancelled("cancelled at authenticate succeeded")),
            Self::Invalid(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<AuthenticateWithTokenResponsePb> for EncodeAuthTokenEvent {
    fn respond_to(self) -> Result<Response<AuthenticateWithTokenResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => {
                Err(Status::cancelled("cancelled at token expires calculated"))
            }
            Self::Success(token, granted) => (token, granted).respond_to(),
            Self::TicketNotFound => Err(Status::unauthenticated("ticket not found")),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<AuthenticateWithTokenResponsePb> for (AuthToken, AuthPermissionGranted) {
    fn respond_to(self) -> Result<Response<AuthenticateWithTokenResponsePb>, Status> {
        Ok(Response::new(AuthenticateWithTokenResponsePb {
            token: Some(self.0.into()),
            granted: Some(self.1.into()),
        }))
    }
}
