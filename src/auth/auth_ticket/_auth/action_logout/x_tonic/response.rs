use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::auth::auth_ticket::_common::y_protobuf::service::LogoutResponsePb;

use super::super::action::LogoutState;

impl RespondTo<LogoutResponsePb> for LogoutState {
    fn respond_to(self) -> Result<Response<LogoutResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::Discard(event) => event.respond_to(),
        }
    }
}
