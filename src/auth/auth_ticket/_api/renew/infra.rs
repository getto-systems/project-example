use crate::auth::auth_ticket::_api::kernel::infra::AuthHeaderInfra;

use crate::{
    auth::{
        _api::service::data::ServiceError,
        auth_ticket::{
            _api::{
                kernel::data::{AuthNonceValue, AuthTokenValue},
                renew::data::AuthTokenMessage,
            },
            _common::encode::data::EncodeAuthTicketResponse,
        },
    },
    z_details::_api::message::data::MessageError,
};

pub trait RenewAuthTicketInfra {
    type HeaderInfra: AuthHeaderInfra;
    type RenewService: RenewAuthTicketService;
    type Messenger: RenewAuthTicketMessenger;

    fn header_infra(&self) -> &Self::HeaderInfra;
    fn renew_service(&self) -> &Self::RenewService;
    fn messenger(&self) -> &Self::Messenger;
}

#[async_trait::async_trait]
pub trait RenewAuthTicketService {
    async fn renew(
        &self,
        nonce: AuthNonceValue,
        token: AuthTokenValue,
    ) -> Result<EncodeAuthTicketResponse, ServiceError>;
}

pub trait RenewAuthTicketMessenger {
    fn encode(&self, response: EncodeAuthTicketResponse) -> Result<AuthTokenMessage, MessageError>;
}
