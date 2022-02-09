use tonic::{Response, Status};

use crate::z_lib::{request::data::MetadataError, response::tonic::ServiceResponder};

impl<T> ServiceResponder<T> for MetadataError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::invalid_argument(format!("{}", self)))
    }
}
