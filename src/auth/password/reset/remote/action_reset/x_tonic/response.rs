use tonic::{Response, Status};

use crate::auth::password::reset::_common::y_protobuf::service::ResetPasswordResponsePb;

use crate::z_details::_common::response::tonic::RespondTo;

use super::super::action::ResetPasswordState;

impl RespondTo<ResetPasswordResponsePb> for ResetPasswordState {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        match self {
            Self::Reset(event) => event.respond_to(),
            Self::Issue(event) => event.respond_to(),
            Self::Encode(event) => event.respond_to(),
        }
    }
}
