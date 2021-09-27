use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::auth::auth_ticket::_common::y_protobuf::service::ValidateApiTokenResponsePb;

use super::super::action::ValidateApiTokenState;

impl RespondTo<ValidateApiTokenResponsePb> for ValidateApiTokenState {
    fn respond_to(self) -> Result<Response<ValidateApiTokenResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::Success(user) => user.respond_to(),
        }
    }
}
