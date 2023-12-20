use tonic::{Response, Status};

use crate::common::api::response::x_tonic::ServiceResponder;

use crate::auth::ticket::issue::data::IssueAuthTicketError;

impl<T> ServiceResponder<T> for IssueAuthTicketError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
