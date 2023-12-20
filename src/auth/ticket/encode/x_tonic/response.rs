use tonic::{Response, Status};

use crate::common::api::response::x_tonic::ServiceResponder;

use crate::auth::ticket::encode::data::{EncodeAuthTokenError, EncodeTokenError};

impl<T> ServiceResponder<T> for EncodeAuthTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::TicketNotFound => Err(Status::unauthenticated("ticket not found")),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

impl<T> ServiceResponder<T> for EncodeTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("{}", self)))
    }
}
