use tonic::{Response, Status};

use crate::auth::auth_ticket::_common::y_protobuf::service::LogoutResponsePb;

use crate::z_details::_common::response::tonic::RespondTo;

use super::super::event::DiscardAuthTicketEvent;

impl RespondTo<LogoutResponsePb> for DiscardAuthTicketEvent {
    fn respond_to(self) -> Result<Response<LogoutResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(LogoutResponsePb {})),
            Self::Validate(event) => event.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
