use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::avail::unexpected_error::_common::y_protobuf::service::NotifyResponsePb;

use super::super::event::NotifyUnexpectedErrorEvent;

impl RespondTo<NotifyResponsePb> for NotifyUnexpectedErrorEvent {
    fn respond_to(self) -> Result<Response<NotifyResponsePb>, Status> {
        match self {
            Self::Error(_) => Ok(Response::new(NotifyResponsePb {})),
            Self::ValidateError(_) => Err(Status::unauthenticated("unauthenticated")),
        }
    }
}
