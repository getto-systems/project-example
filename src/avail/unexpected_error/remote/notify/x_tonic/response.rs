use tonic::{Response, Status};

use crate::avail::unexpected_error::remote::notify::action::NotifyUnexpectedErrorEvent;
use crate::z_lib::remote::response::tonic::RespondTo;

use crate::avail::unexpected_error::remote::y_protobuf::service::NotifyResponsePb;

use super::super::action::NotifyUnexpectedErrorState;

impl RespondTo<NotifyResponsePb> for NotifyUnexpectedErrorState {
    fn respond_to(self) -> Result<Response<NotifyResponsePb>, Status> {
        match self {
            Self::Validate(_) => Err(Status::unauthenticated("unauthenticated")),
            Self::Notify(event) => event.respond_to(),
        }
    }
}

impl RespondTo<NotifyResponsePb> for NotifyUnexpectedErrorEvent {
    fn respond_to(self) -> Result<Response<NotifyResponsePb>, Status> {
        match self {
            Self::Error(_) => Ok(Response::new(NotifyResponsePb {})),
        }
    }
}
