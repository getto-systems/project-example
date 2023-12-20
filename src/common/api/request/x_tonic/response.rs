use tonic::{Response, Status};

use crate::common::api::{request::data::MetadataError, response::x_tonic::ServiceResponder};

impl<T> ServiceResponder<T> for MetadataError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::invalid_argument(format!("{}", self)))
    }
}
