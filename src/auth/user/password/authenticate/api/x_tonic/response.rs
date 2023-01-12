use tonic::{Response, Status};

use crate::auth::user::password::authenticate::y_protobuf::service::AuthenticateWithPasswordResponsePb;

use crate::common::api::response::tonic::ServiceResponder;

use crate::auth::user::password::authenticate::action::{
    AuthenticateWithPasswordEvent, AuthenticateWithPasswordState,
};

use crate::auth::ticket::encode::method::EncodeAuthTokenEvent;

use crate::auth::ticket::kernel::data::{AuthPermissionGranted, AuthToken};

impl ServiceResponder<AuthenticateWithPasswordResponsePb> for AuthenticateWithPasswordState {
    fn respond_to(self) -> Result<Response<AuthenticateWithPasswordResponsePb>, Status> {
        match self {
            Self::AuthenticateWithPassword(event) => event.respond_to(),
            Self::Issue(event) => event.respond_to(),
            Self::Encode(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<AuthenticateWithPasswordResponsePb> for AuthenticateWithPasswordEvent {
    fn respond_to(self) -> Result<Response<AuthenticateWithPasswordResponsePb>, Status> {
        match self {
            Self::Success(_) => Err(Status::cancelled("cancelled at authenticate password")),
            Self::Invalid(_) => Ok(Response::new(failed())),
            Self::NotFound => Ok(Response::new(failed())),
            Self::PasswordNotMatched => Ok(Response::new(failed())),
            Self::PasswordHashError(err) => err.respond_to(),
            Self::RepositoryError(err) => err.respond_to(),
        }
    }
}

fn failed() -> AuthenticateWithPasswordResponsePb {
    AuthenticateWithPasswordResponsePb {
        success: false,
        ..Default::default()
    }
}

impl ServiceResponder<AuthenticateWithPasswordResponsePb> for EncodeAuthTokenEvent {
    fn respond_to(self) -> Result<Response<AuthenticateWithPasswordResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => {
                Err(Status::cancelled("cancelled at token expires calculated"))
            }
            Self::Success(token, granted) => (token, granted).respond_to(),
            Self::TicketNotFound => Err(Status::unauthenticated("ticket not found")),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

impl ServiceResponder<AuthenticateWithPasswordResponsePb> for (AuthToken, AuthPermissionGranted) {
    fn respond_to(self) -> Result<Response<AuthenticateWithPasswordResponsePb>, Status> {
        Ok(Response::new(AuthenticateWithPasswordResponsePb {
            success: true,
            token: Some(self.0.into()),
            granted: Some(self.1.into()),
        }))
    }
}
