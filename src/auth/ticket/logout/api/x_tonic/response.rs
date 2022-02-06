use tonic::{Response, Status};

use crate::z_lib::api::response::tonic::ServiceResponder;

use crate::auth::ticket::logout::y_protobuf::service::LogoutResponsePb;

use crate::auth::ticket::validate::method::ValidateAuthTokenEvent;

use super::super::action::{LogoutEvent, LogoutState};

impl ServiceResponder<LogoutResponsePb> for LogoutState {
    fn respond_to(self) -> Result<Response<LogoutResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::Logout(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<LogoutResponsePb> for ValidateAuthTokenEvent {
    fn respond_to(self) -> Result<Response<LogoutResponsePb>, Status> {
        match self {
            Self::ValidateNonce(_) => Err(Status::cancelled("logout cancelled")),
            Self::Success(_) => Err(Status::cancelled("logout succeeded")),
            Self::TokenNotSent => Err(Status::unauthenticated(format!("{}", self))),
            Self::MetadataError(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<LogoutResponsePb> for LogoutEvent {
    fn respond_to(self) -> Result<Response<LogoutResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(LogoutResponsePb {})),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
