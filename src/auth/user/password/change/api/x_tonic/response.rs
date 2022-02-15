use tonic::{Response, Status};

use crate::auth::user::password::change::y_protobuf::service::ChangePasswordResponsePb;

use crate::z_lib::response::tonic::ServiceResponder;

use super::super::action::{ChangePasswordEvent, ChangePasswordState};

use super::super::data::ChangePasswordError;

impl ServiceResponder<ChangePasswordResponsePb> for ChangePasswordState {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::Change(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<ChangePasswordResponsePb> for ChangePasswordEvent {
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

impl ServiceResponder<ChangePasswordResponsePb> for ChangePasswordError {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        Ok(Response::new(ChangePasswordResponsePb { success: false }))
    }
}
