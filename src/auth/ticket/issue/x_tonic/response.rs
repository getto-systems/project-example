use tonic::{Response, Status};

use crate::z_lib::api::response::tonic::RespondTo;

use super::super::method::IssueAuthTicketEvent;

impl<T> RespondTo<T> for IssueAuthTicketEvent {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::ExpansionLimitCalculated(_) => Err(Status::cancelled("issue cancelled")),
            Self::Success(_) => Err(Status::cancelled("issue cancelled")),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
