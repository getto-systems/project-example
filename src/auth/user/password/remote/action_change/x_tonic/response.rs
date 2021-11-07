use tonic::{Response, Status};

use crate::auth::user::password::remote::y_protobuf::service::ChangePasswordResponsePb;

use crate::z_lib::remote::response::tonic::RespondTo;

use super::super::action::ChangePasswordState;

impl RespondTo<ChangePasswordResponsePb> for ChangePasswordState {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        match self {
            Self::Change(event) => event.respond_to(),
        }
    }
}
