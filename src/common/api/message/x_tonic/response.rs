use tonic::{Response, Status};

use crate::common::api::{message::data::MessageError, response::x_tonic::ServiceResponder};

impl<T> ServiceResponder<T> for MessageError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        match self {
            Self::ProtobufDecodeError(_) => {
                Err(Status::internal(format!("protobuf message error")))
            }
            Self::Invalid(_) => Err(Status::internal(format!("invalid message error"))),
        }
    }
}
