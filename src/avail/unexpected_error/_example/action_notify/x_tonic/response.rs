use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::avail::unexpected_error::_common::y_protobuf::service::NotifyResponsePb;

use super::super::action::NotifyUnexpectedErrorState;

impl RespondTo<NotifyResponsePb> for NotifyUnexpectedErrorState {
    fn respond_to(self) -> Result<Response<NotifyResponsePb>, Status> {
        match self {
            Self::Notify(event) => event.respond_to(),
        }
    }
}
