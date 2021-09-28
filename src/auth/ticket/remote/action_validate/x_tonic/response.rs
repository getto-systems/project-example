use tonic::{Response, Status};

use crate::z_lib::remote::response::tonic::RespondTo;

use crate::auth::ticket::remote::y_protobuf::service::ValidateApiTokenResponsePb;

use super::super::action::ValidateApiTokenState;

impl RespondTo<ValidateApiTokenResponsePb> for ValidateApiTokenState {
    fn respond_to(self) -> Result<Response<ValidateApiTokenResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::Success(user) => user.respond_to(),
        }
    }
}
