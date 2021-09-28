use tonic::{Response, Status};

use crate::auth::user::password::remote::y_protobuf::service::ChangePasswordResponsePb;

use crate::z_lib::remote::response::tonic::RespondTo;

use super::super::event::ChangePasswordEvent;

use crate::auth::user::password::remote::change::data::ChangePasswordError;

impl RespondTo<ChangePasswordResponsePb> for ChangePasswordEvent {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(ChangePasswordResponsePb { success: true })),
            Self::UserNotFound => Err(Status::internal("user not found")),
            Self::Validate(_) => Err(Status::cancelled("change password cancelled")),
            Self::InvalidPassword(err) => err.respond_to(),
            Self::NonceError(err) => err.respond_to(),
            Self::PasswordHashError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}

impl RespondTo<ChangePasswordResponsePb> for ChangePasswordError {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        Ok(Response::new(ChangePasswordResponsePb { success: false }))
    }
}
