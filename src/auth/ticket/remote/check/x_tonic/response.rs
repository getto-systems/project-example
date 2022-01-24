use tonic::{Response, Status};

use crate::auth::ticket::remote::encode::data::AuthTicketEncoded;
use crate::auth::ticket::remote::encode::event::EncodeAuthTicketEvent;
use crate::auth::ticket::remote::kernel::data::AuthTokenEncoded;
use crate::auth::ticket::remote::validate::event::ValidateAuthTokenEvent;
use crate::auth::user::remote::kernel::data::AuthUserExtract;
use crate::z_lib::remote::response::tonic::RespondTo;

use crate::auth::ticket::remote::y_protobuf::service::CheckAuthTicketResponsePb;

use super::super::action::CheckAuthTicketState;

impl RespondTo<CheckAuthTicketResponsePb> for CheckAuthTicketState {
    fn respond_to(self) -> Result<Response<CheckAuthTicketResponsePb>, Status> {
        match self {
            Self::Validate(event) => event.respond_to(),
            Self::Encode(event) => event.respond_to(),
        }
    }
}

impl RespondTo<CheckAuthTicketResponsePb> for AuthTicketEncoded {
    fn respond_to(self) -> Result<Response<CheckAuthTicketResponsePb>, Status> {
        Ok(Response::new(CheckAuthTicketResponsePb {
            user: Some(self.user.into()),
            token: Some(self.token.into()),
        }))
    }
}

impl Into<Option<AuthTicketEncoded>> for CheckAuthTicketResponsePb {
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

impl RespondTo<CheckAuthTicketResponsePb> for EncodeAuthTicketEvent {
    fn respond_to(self) -> Result<Response<CheckAuthTicketResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => Err(Status::cancelled("token expires calculated")),
            Self::Success(response) => response.respond_to(),
            Self::TicketNotFound => Err(Status::unauthenticated("ticket not found")),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

impl RespondTo<CheckAuthTicketResponsePb> for ValidateAuthTokenEvent {
    fn respond_to(self) -> Result<Response<CheckAuthTicketResponsePb>, Status> {
        match self {
            Self::Success(_) => Err(Status::cancelled("check auth ticket cancelled")),
            Self::NonceError(err) => err.respond_to(),
            Self::TokenError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}