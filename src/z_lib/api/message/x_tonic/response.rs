use tonic::{Response, Status};

use crate::z_lib::{message::data::MessageError, response::tonic::ServiceResponder};

impl<T> ServiceResponder<T> for MessageError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::Invalid(message) => Err(Status::internal(message)),
        }
    }
}