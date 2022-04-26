use tonic::{Response, Status};

use crate::auth::ticket::validate::y_protobuf::service::AuthorizeResponsePb;

use crate::z_lib::response::tonic::ServiceResponder;

use crate::auth::ticket::validate::action::AuthenticateApiState;

use crate::auth::ticket::validate::method::{
    AuthorizeEvent, ValidateAuthNonceEvent, AuthenticateEvent,
};

impl ServiceResponder<AuthorizeResponsePb> for AuthenticateApiState {
    fn respond_to(self) -> Result<Response<AuthorizeResponsePb>, Status> {
        match self {
            Self::Authenticate(event) => event.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
            Self::Success(user) => user.respond_to(),
        }
    }
}

impl<T> ServiceResponder<T> for AuthenticateEvent {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::ValidateNonce(event) => event.respond_to(),
            Self::Success(_) => Err(Status::cancelled("cancelled at authenticate succeeded")),
            Self::TokenNotSent => Err(Status::unauthenticated(format!("{}", self))),
            Self::MetadataError(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
        }
    }
}

impl<T> ServiceResponder<T> for AuthorizeEvent {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::Success => Err(Status::cancelled("cancelled at authorize succeeded")),
            Self::ServiceError(err) => err.respond_to(),
            Self::MetadataError(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
        }
    }
}

impl<T> ServiceResponder<T> for ValidateAuthNonceEvent {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::NonceExpiresCalculated(_) => {
                Err(Status::cancelled("cancelled at nonce expires calculated"))
            }
            Self::Success => Err(Status::cancelled("cancelled at validate nonce succeeded")),
            Self::NonceNotSent => Err(Status::invalid_argument(format!("{}", self))),
            Self::MetadataError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::Conflict => Err(Status::already_exists(format!("{}", self))),
        }
    }
}
