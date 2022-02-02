use tonic::{Response, Status};

use crate::auth::{
    ticket::remote::y_protobuf::service::EncodedAuthTokensPb,
    user::password::remote::y_protobuf::service::{
        AuthenticatePasswordMaskedResponsePb, AuthenticatePasswordResponsePb,
    },
};

use crate::z_lib::remote::response::tonic::RespondTo;

use super::super::action::{AuthenticatePasswordEvent, AuthenticatePasswordState};

use crate::auth::ticket::encode::method::EncodeAuthTicketEvent;

use crate::auth::{
    ticket::encode::data::AuthTicketEncoded,
    user::password::authenticate::remote::data::AuthenticatePasswordError,
};

impl RespondTo<AuthenticatePasswordResponsePb> for AuthenticatePasswordState {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        match self {
            Self::Authenticate(event) => event.respond_to(),
            Self::ValidateNonce(event) => event.respond_to(),
            Self::Issue(event) => event.respond_to(),
            Self::Encode(event) => event.respond_to(),
        }
    }
}

impl RespondTo<AuthenticatePasswordResponsePb> for EncodeAuthTicketEvent {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => Err(Status::cancelled("token expires calculated")),
            Self::Success(response) => response.respond_to(),
            Self::TicketNotFound => Err(Status::unauthenticated("ticket not found")),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

impl AuthenticatePasswordResponsePb {
    pub fn extract(
        self,
    ) -> (
        Option<EncodedAuthTokensPb>,
        AuthenticatePasswordMaskedResponsePb,
    ) {
        (
            self.token,
            AuthenticatePasswordMaskedResponsePb {
                success: self.success,
                roles: self.roles,
            },
        )
    }
}

impl RespondTo<AuthenticatePasswordResponsePb> for AuthTicketEncoded {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        Ok(Response::new(AuthenticatePasswordResponsePb {
            success: true,
            roles: Some(self.roles.into()),
            token: Some(self.token.into()),
        }))
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