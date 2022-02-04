use tonic::{Response, Status};

use crate::z_lib::api::response::tonic::RespondTo;

use crate::auth::ticket::y_protobuf::service::{
    CheckAuthTicketMaskedResponsePb, CheckAuthTicketResponsePb, EncodedAuthTokensPb,
};

use super::super::action::CheckAuthTicketState;

use crate::auth::ticket::{
    encode::method::EncodeAuthTicketEvent, validate::method::ValidateAuthTokenEvent,
};

use crate::auth::ticket::encode::data::AuthTicketEncoded;

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
            roles: Some(self.roles.into()),
            token: Some(self.token.into()),
        }))
    }
}

impl CheckAuthTicketResponsePb {
    pub fn extract(self) -> (Option<EncodedAuthTokensPb>, CheckAuthTicketMaskedResponsePb) {
        (
            self.token,
            CheckAuthTicketMaskedResponsePb { roles: self.roles },
        )
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
            Self::ValidateNonce(_) => Err(Status::cancelled("check auth ticket cancelled")),
            Self::Success(_) => Err(Status::cancelled("check auth ticket succeeded")),
            Self::TokenNotSent => Err(Status::unauthenticated(format!("{}", self))),
            Self::MetadataError(err) => err.respond_to(),
            Self::DecodeError(err) => err.respond_to(),
            Self::PermissionError(err) => err.respond_to(),
        }
    }
}
