use tonic::{Response, Status};

use crate::z_lib::response::tonic::ServiceResponder;

use crate::auth::ticket::{
    check::y_protobuf::service::{CheckAuthTicketMaskedResponsePb, CheckAuthTicketResponsePb},
    y_protobuf::service::EncodedAuthTokensPb,
};

use super::super::action::CheckAuthTicketState;

use crate::auth::ticket::encode::method::EncodeAuthTicketEvent;

use crate::auth::ticket::encode::data::AuthTicketEncoded;

impl ServiceResponder<CheckAuthTicketResponsePb> for CheckAuthTicketState {
    fn respond_to(self) -> Result<Response<CheckAuthTicketResponsePb>, Status> {
        match self {
            Self::Authenticate(event) => event.respond_to(),
            Self::Encode(event) => event.respond_to(),
        }
    }
}

impl ServiceResponder<CheckAuthTicketResponsePb> for AuthTicketEncoded {
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

impl ServiceResponder<CheckAuthTicketResponsePb> for EncodeAuthTicketEvent {
    fn respond_to(self) -> Result<Response<CheckAuthTicketResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => Err(Status::cancelled("cancelled at token expires calculated")),
            Self::Success(response) => response.respond_to(),
            Self::TicketNotFound => Err(Status::unauthenticated("ticket not found")),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}
