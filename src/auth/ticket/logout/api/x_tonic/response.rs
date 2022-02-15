use tonic::{Response, Status};

use crate::z_lib::response::tonic::ServiceResponder;

use crate::auth::ticket::logout::y_protobuf::service::LogoutResponsePb;

use super::super::action::{LogoutEvent, LogoutState};

impl ServiceResponder<LogoutResponsePb> for LogoutState {
    fn respond_to(self) -> Result<Response<LogoutResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::Logout(event) => event.respond_to(),
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
