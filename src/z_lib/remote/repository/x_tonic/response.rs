use tonic::{Response, Status};

use crate::z_lib::remote::response::tonic::RespondTo;

use crate::z_lib::remote::repository::data::RepositoryError;

impl<T> RespondTo<T> for RepositoryError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("{}", self)))
    }
}
