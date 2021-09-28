use tonic::{Response, Status};

use crate::z_lib::remote::response::tonic::RespondTo;

use crate::auth::ticket::_common::y_protobuf::service::LogoutResponsePb;

use super::super::action::LogoutState;

impl RespondTo<LogoutResponsePb> for LogoutState {
    fn respond_to(self) -> Result<Response<LogoutResponsePb>, Status> {
        match self {
            Self::Discard(event) => event.respond_to(),
        }
    }
}
