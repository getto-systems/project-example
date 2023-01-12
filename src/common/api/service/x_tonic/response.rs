use tonic::{Response, Status};

use crate::common::api::response::tonic::ServiceResponder;

use crate::common::api::service::data::{ServiceConnectError, ServiceMetadataError};

impl<T> ServiceResponder<T> for ServiceConnectError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("{}", self)))
    }
}

impl<T> ServiceResponder<T> for ServiceMetadataError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("{}", self)))
    }
}
