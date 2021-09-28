use tonic::{Response, Status};

use crate::auth::user::password::reset::remote::y_protobuf::service::RequestResetTokenResponsePb;

use crate::z_lib::remote::response::tonic::RespondTo;

use super::super::action::RequestResetTokenState;

impl RespondTo<RequestResetTokenResponsePb> for RequestResetTokenState {
    fn respond_to(self) -> Result<Response<RequestResetTokenResponsePb>, Status> {
        match self {
            Self::RequestToken(event) => event.respond_to(),
        }
    }
}
