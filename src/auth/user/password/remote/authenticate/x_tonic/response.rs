use tonic::{Response, Status};

use crate::auth::user::password::remote::y_protobuf::service::AuthenticatePasswordResponsePb;

use crate::z_lib::remote::response::tonic::RespondTo;

use super::super::action::{AuthenticatePasswordEvent, AuthenticatePasswordState};

use super::super::data::AuthenticatePasswordError;

impl RespondTo<AuthenticatePasswordResponsePb> for AuthenticatePasswordState {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        match self {
            Self::Authenticate(event) => event.respond_to(),
            Self::Nonce(err) => err.respond_to(),
            Self::Issue(event) => event.respond_to(),
            Self::Encode(event) => event.respond_to(),
        }
    }
}

impl RespondTo<AuthenticatePasswordResponsePb> for AuthenticatePasswordEvent {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        match self {
            Self::Success(_) => Err(Status::cancelled("authenticate password cancelled")),
            Self::UserNotFound => Err(Status::internal("user not found")),
            Self::InvalidPassword(err) => err.respond_to(),
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
