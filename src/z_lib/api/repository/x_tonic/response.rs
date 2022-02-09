use tonic::{Response, Status};

use crate::z_lib::response::tonic::ServiceResponder;

use crate::z_lib::repository::data::RepositoryError;

impl<T> ServiceResponder<T> for RepositoryError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("{}", self)))
    }
}
