use tonic::{Response, Status};

use crate::auth::{
    ticket::y_protobuf::service::EncodedAuthTokensPb,
    user::password::authenticate::y_protobuf::service::{
        AuthenticatePasswordMaskedResponsePb, AuthenticatePasswordResponsePb,
    },
};

use crate::z_lib::response::tonic::ServiceResponder;

use super::super::action::{AuthenticatePasswordEvent, AuthenticatePasswordState};

use crate::auth::ticket::encode::method::EncodeAuthTicketEvent;

use crate::auth::ticket::encode::data::AuthTicketEncoded;

impl ServiceResponder<AuthenticatePasswordResponsePb> for AuthenticatePasswordState {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        match self {
            Self::Authenticate(event) => event.respond_to(),
            Self::ValidateNonce(event) => event.respond_to(),
            Self::Issue(event) => event.respond_to(),
            Self::Encode(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<AuthenticatePasswordResponsePb> for EncodeAuthTicketEvent {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => {
                Err(Status::cancelled("cancelled at token expires calculated"))
            }
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

impl ServiceResponder<AuthenticatePasswordResponsePb> for AuthTicketEncoded {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        Ok(Response::new(AuthenticatePasswordResponsePb {
            success: true,
            roles: Some(self.roles.into()),
            token: Some(self.token.into()),
        }))
    }
}

impl ServiceResponder<AuthenticatePasswordResponsePb> for AuthenticatePasswordEvent {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        match self {
            Self::Success(_) => Err(Status::cancelled("cancelled at authenticate password")),
            Self::Invalid(_) => Ok(Response::new(AuthenticatePasswordResponsePb {
                success: false,
                ..Default::default()
            })),
            Self::NotFound => Ok(Response::new(AuthenticatePasswordResponsePb {
                success: false,
                ..Default::default()
            })),
            Self::PasswordNotMatched => Ok(Response::new(AuthenticatePasswordResponsePb {
                success: false,
                ..Default::default()
            })),
            Self::PasswordHashError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}
