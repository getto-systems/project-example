use tonic::{Response, Status};

use crate::z_lib::remote::response::tonic::RespondTo;

use crate::auth::ticket::remote::y_protobuf::service::LogoutResponsePb;

use super::super::action::{LogoutEvent, LogoutState};

impl RespondTo<LogoutResponsePb> for LogoutState {
    fn respond_to(self) -> Result<Response<LogoutResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::Logout(event) => event.respond_to(),
        }
    }
}

impl RespondTo<LogoutResponsePb> for LogoutEvent {
    fn respond_to(self) -> Result<Response<LogoutResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(LogoutResponsePb {})),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
