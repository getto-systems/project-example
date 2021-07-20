use tonic::{Response, Status};

use crate::auth::password::_common::y_protobuf::service::{
    AuthenticatePasswordRequestPb, AuthenticatePasswordResponsePb,
};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::auth::password::_auth::authenticate::infra::AuthenticatePasswordFieldsExtract;

use super::super::event::AuthenticatePasswordEvent;

use crate::auth::password::_auth::authenticate::data::AuthenticatePasswordError;

impl RespondTo<AuthenticatePasswordResponsePb> for AuthenticatePasswordEvent {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        match self {
            Self::Success(_) => Err(Status::cancelled("authenticate password cancelled")),
            Self::UserNotFound => Err(Status::internal("user not found")),
            Self::InvalidPassword(err) => err.respond_to(),
            Self::NonceError(err) => err.respond_to(),
            Self::PasswordHashError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}

impl RespondTo<AuthenticatePasswordResponsePb> for AuthenticatePasswordError {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        Ok(Response::new(AuthenticatePasswordResponsePb {
            success: false,
            ..Default::default()
        }))
    }
}

impl Into<AuthenticatePasswordFieldsExtract> for AuthenticatePasswordRequestPb {
    fn into(self) -> AuthenticatePasswordFieldsExtract {
        AuthenticatePasswordFieldsExtract {
            login_id: self.login_id,
            password: self.password,
        }
    }
}
