use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::auth::{
    ticket::_common::y_protobuf::service::RenewAuthTicketResponsePb,
    user::password::{
        _common::y_protobuf::service::AuthenticatePasswordResponsePb,
        reset::_common::y_protobuf::service::ResetPasswordResponsePb,
    },
};

use crate::auth::ticket::remote::encode::event::EncodeAuthTicketEvent;

use crate::auth::{
    ticket::remote::{
        encode::data::{AuthTicketEncoded, EncodeAuthTokenError},
        kernel::data::AuthTokenEncoded,
    },
    user::remote::kernel::data::AuthUserExtract,
};

impl RespondTo<RenewAuthTicketResponsePb> for AuthTicketEncoded {
    fn respond_to(self) -> Result<Response<RenewAuthTicketResponsePb>, Status> {
        Ok(Response::new(RenewAuthTicketResponsePb {
            user: Some(self.user.into()),
            token: Some(self.token.into()),
        }))
    }
}

impl RespondTo<AuthenticatePasswordResponsePb> for AuthTicketEncoded {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        Ok(Response::new(AuthenticatePasswordResponsePb {
            success: true,
            user: Some(self.user.into()),
            token: Some(self.token.into()),
        }))
    }
}

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

impl Into<Option<AuthTicketEncoded>> for RenewAuthTicketResponsePb {
    fn into(self) -> Option<AuthTicketEncoded> {
        match (self.user, self.token) {
            (Some(user), Some(token)) => {
                let user: AuthUserExtract = user.into();
                let token: Option<AuthTokenEncoded> = token.into();
                token.map(|token| AuthTicketEncoded { user, token })
            }
            _ => None,
        }
    }
}

impl RespondTo<RenewAuthTicketResponsePb> for EncodeAuthTicketEvent {
    fn respond_to(self) -> Result<Response<RenewAuthTicketResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => token_expires_calculated(),
            Self::Success(response) => response.respond_to(),
            Self::TicketNotFound => ticket_not_found(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

impl RespondTo<AuthenticatePasswordResponsePb> for EncodeAuthTicketEvent {
    fn respond_to(self) -> Result<Response<AuthenticatePasswordResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => token_expires_calculated(),
            Self::Success(response) => response.respond_to(),
            Self::TicketNotFound => ticket_not_found(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

impl RespondTo<ResetPasswordResponsePb> for EncodeAuthTicketEvent {
    fn respond_to(self) -> Result<Response<ResetPasswordResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => token_expires_calculated(),
            Self::Success(response) => response.respond_to(),
            Self::TicketNotFound => ticket_not_found(),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

fn token_expires_calculated<T>() -> Result<Response<T>, Status> {
    Err(Status::cancelled("renew auth ticket cancelled"))
}
fn ticket_not_found<T>() -> Result<Response<T>, Status> {
    Err(Status::unauthenticated("ticket not found"))
}

impl<T> RespondTo<T> for EncodeAuthTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("{}", self)))
    }
}
