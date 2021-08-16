use crate::auth::auth_ticket::_api::kernel::infra::{
    AuthNonceHeader, AuthTokenHeader, AuthTokenInfra,
};

use crate::{
    auth::{
        _api::service::data::ServiceError,
        auth_ticket::{
            _api::kernel::data::AuthTokenMessage,
            _common::{
                encode::data::AuthTicketEncoded,
                kernel::data::{AuthNonceValue, AuthTokenValue},
            },
        },
    },
    z_details::_api::message::data::MessageError,
};

pub trait RenewAuthTicketInfra {
    type NonceHeader: AuthNonceHeader;
    type TokenHeader: AuthTokenHeader;
    type TokenInfra: AuthTokenInfra;
    type RenewService: RenewAuthTicketService;
    type ResponseEncoder: RenewAuthTicketResponseEncoder;

    fn nonce_header(&self) -> &Self::NonceHeader;
    fn token_header(&self) -> &Self::TokenHeader;
    fn token_infra(&self) -> &Self::TokenInfra;
    fn renew_service(&self) -> &Self::RenewService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
}

#[async_trait::async_trait]
pub trait RenewAuthTicketService {
    async fn renew(
        &self,
        nonce: AuthNonceValue,
        token: AuthTokenValue,
    ) -> Result<AuthTicketEncoded, ServiceError>;
}

pub trait RenewAuthTicketResponseEncoder {
    fn encode(&self, response: AuthTicketEncoded) -> Result<AuthTokenMessage, MessageError>;
}
