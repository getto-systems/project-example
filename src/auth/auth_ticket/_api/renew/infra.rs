use crate::auth::auth_ticket::_api::kernel::infra::{AuthHeaderInfra, AuthTokenInfra};

use crate::{
    auth::{
        _api::service::data::ServiceError,
        auth_ticket::{
            _api::kernel::data::AuthTokenMessageEncoded,
            _common::{
                encode::data::AuthTicketEncoded,
                kernel::data::{AuthNonceValue, AuthTokenValue},
            },
        },
    },
    z_details::_api::message::data::MessageError,
};

pub trait RenewAuthTicketInfra {
    type HeaderInfra: AuthHeaderInfra;
    type TokenInfra: AuthTokenInfra;
    type RenewService: RenewAuthTicketService;
    type ResponseEncoder: RenewAuthTicketResponseEncoder;

    fn header_infra(&self) -> &Self::HeaderInfra;
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
    fn encode(
        &self,
        response: AuthTicketEncoded,
    ) -> Result<AuthTokenMessageEncoded, MessageError>;
}
