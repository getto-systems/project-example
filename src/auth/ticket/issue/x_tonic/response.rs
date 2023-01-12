use tonic::{Response, Status};

use crate::common::api::response::tonic::ServiceResponder;

use super::super::method::IssueAuthTicketEvent;

impl<T> ServiceResponder<T> for IssueAuthTicketEvent {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::ExpansionLimitCalculated(_) => {
                Err(Status::cancelled("cancelled at expansion limit calculated"))
            }
            Self::Success(_) => Err(Status::cancelled("cancelled at ticket issued")),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
