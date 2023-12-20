use tonic::{Response, Status};

use crate::common::api::response::x_tonic::ServiceResponder;

use crate::auth::ticket::authorize::{
    data::{AuthorizeError, CheckAuthorizeTokenError},
    proxy::data::AuthorizeProxyError,
};

impl<R> ServiceResponder<R> for CheckAuthorizeTokenError {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::Invalid(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}

impl<R> ServiceResponder<R> for AuthorizeError {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::TicketNotFound => Err(Status::internal("ticket not found")),
            Self::TicketHasExpired => Err(Status::internal("ticket has expired")),
            Self::UserNotFound => Err(Status::internal("user not found")),
            Self::Invalid(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}

impl<R> ServiceResponder<R> for AuthorizeProxyError {
    fn respond_to(self) -> Result<Response<R>, Status> {
        match self {
            Self::Invalid(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
            Self::ProxyError(err) => err.respond_to(),
        }
    }
}
