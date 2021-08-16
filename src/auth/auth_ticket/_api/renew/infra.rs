use crate::auth::auth_ticket::_api::kernel::infra::{
    AuthNonceHeader, AuthTokenHeader, AuthTokenResponseBuilder,
};

use crate::{
    auth::{
        _api::service::data::AuthServiceError,
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
    type RenewService: RenewAuthTicketService;
    type ResponseEncoder: RenewAuthTicketResponseEncoder;
    type ResponseBuilder: AuthTokenResponseBuilder;

    fn nonce_header(&self) -> &Self::NonceHeader;
    fn token_header(&self) -> &Self::TokenHeader;
    fn renew_service(&self) -> &Self::RenewService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
    fn response_builder(&self) -> &Self::ResponseBuilder;
}

#[async_trait::async_trait]
pub trait RenewAuthTicketService {
    async fn renew(
        &self,
        nonce: Option<AuthNonceValue>,
        token: Option<AuthTokenValue>,
    ) -> Result<AuthTicketEncoded, AuthServiceError>;
}

pub trait RenewAuthTicketResponseEncoder {
    fn encode(&self, response: AuthTicketEncoded) -> Result<AuthTokenMessage, MessageError>;
}
