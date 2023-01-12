use tonic::{Response, Status};

use crate::common::api::response::tonic::ServiceResponder;

use crate::common::api::repository::data::RepositoryError;

impl<T> ServiceResponder<T> for RepositoryError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("{}", self)))
    }
}
