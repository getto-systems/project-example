use std::iter::FromIterator;

use tonic::{Response, Status};

use crate::z_details::_common::response::tonic::RespondTo;

use crate::auth::auth_ticket::_auth::y_protobuf::service::{
    AuthTokenKindPb, AuthTokenPb, RenewAuthTicketResponsePb,
};

use crate::auth::auth_ticket::_auth::encode::event::EncodeAuthTicketEvent;

use crate::auth::auth_ticket::_auth::encode::data::{
    AuthTokenEncoded, AuthTokenEncodedData, AuthTokenKind, EncodeAuthTokenError,
};

impl RespondTo<RenewAuthTicketResponsePb> for EncodeAuthTicketEvent {
    fn respond_to(self) -> Result<Response<RenewAuthTicketResponsePb>, Status> {
        match self {
            Self::TokenExpiresCalculated(_) => {
                Err(Status::cancelled("renew auth ticket cancelled"))
            }
            Self::Success(token) => token.respond_to(),
            Self::TicketNotFound => Err(Status::unauthenticated("ticket not found")),
            Self::RepositoryError(err) => err.respond_to(),
            Self::EncodeError(err) => err.respond_to(),
        }
    }
}

impl RespondTo<RenewAuthTicketResponsePb> for AuthTokenEncoded {
    fn respond_to(self) -> Result<Response<RenewAuthTicketResponsePb>, Status> {
        Ok(Response::new(RenewAuthTicketResponsePb {
            granted_roles: Vec::from_iter(self.granted_roles.extract().into_iter()),

            ticket_tokens: self
                .ticket_tokens
                .into_iter()
                .map(|token| token.into())
                .collect(),

            api_tokens: self
                .api_tokens
                .into_iter()
                .map(|token| token.into())
                .collect(),

            cloudfront_tokens: self
                .cloudfront_tokens
                .into_iter()
                .map(|token| token.into())
                .collect(),
        }))
    }
}

impl Into<AuthTokenPb> for AuthTokenEncodedData {
    fn into(self) -> AuthTokenPb {
        let kind: AuthTokenKindPb = self.kind.into();

        AuthTokenPb {
            kind: kind as i32,
            token: self.token.value,
            expires: self.token.expires.extract().timestamp(),
        }
    }
}

impl Into<AuthTokenKindPb> for AuthTokenKind {
    fn into(self) -> AuthTokenKindPb {
        match self {
            Self::Ticket => AuthTokenKindPb::Ticket,
            Self::Api => AuthTokenKindPb::Api,
            Self::CloudfrontKeyPairId => AuthTokenKindPb::CloudfrontKeyPairId,
            Self::CloudfrontPolicy => AuthTokenKindPb::CloudfrontPolicy,
            Self::CloudfrontSignature => AuthTokenKindPb::CloudfrontSignature,
        }
    }
}

impl<T> RespondTo<T> for EncodeAuthTokenError {
    fn respond_to(self) -> Result<Response<T>, Status> {
        Err(Status::internal(format!("{}", self)))
    }
}
