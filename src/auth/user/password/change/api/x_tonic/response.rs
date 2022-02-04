use tonic::{Response, Status};

use crate::auth::user::password::change::y_protobuf::service::ChangePasswordResponsePb;

use crate::z_lib::api::response::tonic::RespondTo;

use super::super::action::{ChangePasswordEvent, ChangePasswordState};

use super::super::data::ChangePasswordError;

impl RespondTo<ChangePasswordResponsePb> for ChangePasswordState {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        match self {
            Self::Validate(_) => Err(Status::permission_denied("permission denied")),
            Self::Change(event) => event.respond_to(),
        }
    }
}

impl RespondTo<ChangePasswordResponsePb> for ChangePasswordEvent {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(ChangePasswordResponsePb { success: true })),
            Self::UserNotFound => Err(Status::internal("user not found")),
            Self::InvalidPassword(err) => err.respond_to(),
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
