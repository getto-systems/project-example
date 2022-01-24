use tonic::{Response, Status};

use crate::z_lib::remote::response::tonic::RespondTo;

use crate::auth::user::password::reset::remote::y_protobuf::service::ResetPasswordResponsePb;

use crate::auth::ticket::remote::encode::event::EncodeAuthTicketEvent;

use crate::auth::ticket::remote::encode::data::{AuthTicketEncoded, EncodeAuthTokenError};

impl RespondTo<ResetPasswordResponsePb> for AuthTicketEncoded {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        Ok(Response::new(ResetPasswordResponsePb {
            success: true,
            user: Some(self.user.into()),
            token: Some(self.token.into()),
            ..Default::default()
        }))
    }
}

impl RespondTo<ResetPasswordResponsePb> for EncodeAuthTicketEvent {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => Err(Status::cancelled("token expires calculated")),
            Self::Success(response) => response.respond_to(),
            Self::TicketNotFound => Err(Status::unauthenticated("ticket not found")),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

impl<T> RespondTo<T> for EncodeAuthTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("{}", self)))
    }
}
