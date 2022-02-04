use tonic::{Response, Status};

use crate::avail::unexpected_error::notify::y_protobuf::service::NotifyResponsePb;

use crate::z_lib::api::response::tonic::RespondTo;

use crate::avail::unexpected_error::notify::api::action::{
    NotifyUnexpectedErrorEvent, NotifyUnexpectedErrorState,
};

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
