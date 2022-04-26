use tonic::{Response, Status};

use crate::auth::user::password::change::y_protobuf::service::{
    ChangePasswordResponsePb, OverridePasswordResponsePb,
};

use crate::z_lib::response::tonic::ServiceResponder;

use crate::auth::user::password::change::action::{
    ChangePasswordEvent, ChangePasswordState, OverridePasswordEvent, OverridePasswordState,
};

use crate::auth::user::password::change::data::ValidateChangePasswordFieldsError;

impl ServiceResponder<ChangePasswordResponsePb> for ChangePasswordState {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        match self {
            Self::Authenticate(event) => event.respond_to(),
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

impl ServiceResponder<OverridePasswordResponsePb> for OverridePasswordState {
    fn respond_to(self) -> Result<Response<OverridePasswordResponsePb>, Status> {
        match self {
            Self::Authenticate(event) => event.respond_to(),
            Self::Override(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<OverridePasswordResponsePb> for OverridePasswordEvent {
    fn respond_to(self) -> Result<Response<OverridePasswordResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(OverridePasswordResponsePb { success: true })),
            Self::Invalid(_) => Ok(Response::new(OverridePasswordResponsePb { success: false })),
            Self::NotFound => Ok(Response::new(OverridePasswordResponsePb { success: false })),
            Self::PasswordHashError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
