use tonic::{Response, Status};

use crate::z_lib::remote::{request::data::MetadataError, response::tonic::RespondTo};

impl<T> RespondTo<T> for MetadataError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::invalid_argument(format!("{}", self)))
    }
}