use tonic::{Response, Status};

use crate::auth::user::password::remote::y_protobuf::service::AuthenticatePasswordResponsePb;

use crate::z_lib::remote::response::tonic::RespondTo;

use super::super::action::AuthenticatePasswordState;

impl RespondTo<AuthenticatePasswordResponsePb> for AuthenticatePasswordState {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        match self {
            Self::Authenticate(event) => event.respond_to(),
            Self::Issue(event) => event.respond_to(),
            Self::Encode(event) => event.respond_to(),
        }
    }
}
