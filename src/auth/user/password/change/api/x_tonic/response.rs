use tonic::{Response, Status};

use crate::auth::user::password::change::y_protobuf::service::{
    ChangePasswordResponsePb, OverwritePasswordResponsePb,
};

use crate::common::api::response::tonic::ServiceResponder;

use crate::auth::user::password::change::action::{
    ChangePasswordEvent, ChangePasswordState, OverwritePasswordEvent, OverwritePasswordState,
};

use crate::auth::user::password::change::data::ValidateChangePasswordFieldsError;

impl ServiceResponder<ChangePasswordResponsePb> for ChangePasswordState {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        match self {
            Self::Authorize(event) => event.respond_to(),
            Self::Change(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<ChangePasswordResponsePb> for ChangePasswordEvent {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(ChangePasswordResponsePb { success: true })),
            Self::Invalid(err) => err.respond_to(),
            Self::NotFound => Ok(Response::new(ChangePasswordResponsePb { success: false })),
            Self::PasswordNotMatched => {
                Ok(Response::new(ChangePasswordResponsePb { success: false }))
            }
            Self::PasswordHashError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<ChangePasswordResponsePb> for ValidateChangePasswordFieldsError {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        Ok(Response::new(ChangePasswordResponsePb { success: false }))
    }
}

impl ServiceResponder<OverwritePasswordResponsePb> for OverwritePasswordState {
    fn respond_to(self) -> Result<Response<OverwritePasswordResponsePb>, Status> {
        match self {
            Self::Authorize(event) => event.respond_to(),
            Self::Overwrite(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<OverwritePasswordResponsePb> for OverwritePasswordEvent {
    fn respond_to(self) -> Result<Response<OverwritePasswordResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(OverwritePasswordResponsePb { success: true })),
            Self::Invalid(_) => Ok(Response::new(OverwritePasswordResponsePb {
                success: false,
            })),
            Self::NotFound => Ok(Response::new(OverwritePasswordResponsePb {
                success: false,
            })),
            Self::PasswordHashError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
