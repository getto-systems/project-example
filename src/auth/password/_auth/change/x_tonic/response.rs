use tonic::{Response, Status};

use crate::auth::auth_user::_common::kernel::data::AuthUserId;
use crate::auth::password::_common::y_protobuf::service::{
    ChangePasswordRequestPb, ChangePasswordResponsePb,
};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::auth::password::_common::change::infra::ChangePasswordFieldsExtract;

use super::super::event::ChangePasswordEvent;

use crate::auth::password::_auth::change::data::ChangePasswordError;

impl RespondTo<ChangePasswordResponsePb> for ChangePasswordEvent {
    fn respond_to(self) -> Result<Response<ChangePasswordResponsePb>, Status> {
        match self {
            Self::Success => Ok(Response::new(ChangePasswordResponsePb { success: true })),
            Self::UserNotFound => Err(Status::internal("user not found")),
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

impl Into<(AuthUserId, ChangePasswordFieldsExtract)> for ChangePasswordRequestPb {
    fn into(self) -> (AuthUserId, ChangePasswordFieldsExtract) {
        (
            AuthUserId::restore(self.user_id),
            ChangePasswordFieldsExtract {
                current_password: self.current_password,
                new_password: self.new_password,
            },
        )
    }
}
