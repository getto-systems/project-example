use crate::auth::auth_ticket::{
    _api::kernel::infra::AuthTokenResponseBuilder,
    _common::kernel::infra::{AuthNonceMetadata, AuthTokenMetadata},
};

use crate::{
    auth::{
        _common::service::data::AuthServiceError,
        auth_ticket::{
            _api::kernel::data::AuthTokenMessage,
            _common::{
                encode::data::AuthTicketEncoded,
                kernel::data::{AuthNonce, AuthToken},
            },
        },
    },
    z_details::_api::message::data::MessageError,
};

pub trait RenewAuthTicketInfra {
    type NonceMetadata: AuthNonceMetadata;
    type TokenMetadata: AuthTokenMetadata;
    type RenewService: RenewAuthTicketService;
    type ResponseEncoder: RenewAuthTicketResponseEncoder;
    type ResponseBuilder: AuthTokenResponseBuilder;

    fn nonce_metadata(&self) -> &Self::NonceMetadata;
    fn token_metadata(&self) -> &Self::TokenMetadata;
    fn renew_service(&self) -> &Self::RenewService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
    fn response_builder(&self) -> &Self::ResponseBuilder;
}

#[async_trait::async_trait]
pub trait RenewAuthTicketService {
    async fn renew(
        &self,
        nonce: Option<AuthNonce>,
        token: Option<AuthToken>,
    ) -> Result<AuthTicketEncoded, AuthServiceError>;
}

pub trait RenewAuthTicketResponseEncoder {
    fn encode(&self, response: AuthTicketEncoded) -> Result<AuthTokenMessage, MessageError>;
}
