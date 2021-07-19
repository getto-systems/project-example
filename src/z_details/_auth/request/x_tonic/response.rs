use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use super::super::data::MetadataError;

impl<T> RespondTo<T> for MetadataError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::invalid_argument(format!("{}", self)))
    }
}
