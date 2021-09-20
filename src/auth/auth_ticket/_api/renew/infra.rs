use crate::auth::auth_ticket::{
    _api::kernel::infra::AuthTokenResponseBuilder,
    _common::kernel::infra::{AuthMetadata, AuthMetadataContent},
};

use crate::{
    auth::{
        _common::service::data::AuthServiceError,
        auth_ticket::{
            _api::kernel::data::AuthTokenMessage, _common::encode::data::AuthTicketEncoded,
        },
    },
    z_details::_api::message::data::MessageError,
};

pub trait RenewAuthTicketInfra {
    type AuthMetadata: AuthMetadata;
    type RenewService: RenewAuthTicketService;
    type ResponseEncoder: RenewAuthTicketResponseEncoder;
    type ResponseBuilder: AuthTokenResponseBuilder;

    fn auth_metadata(&self) -> &Self::AuthMetadata;
    fn renew_service(&self) -> &Self::RenewService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
    fn response_builder(&self) -> &Self::ResponseBuilder;
}

#[async_trait::async_trait]
pub trait RenewAuthTicketService {
    async fn renew(
        &self,
        metadata: AuthMetadataContent,
    ) -> Result<AuthTicketEncoded, AuthServiceError>;
}

pub trait RenewAuthTicketResponseEncoder {
    fn encode(&self, response: AuthTicketEncoded) -> Result<AuthTokenMessage, MessageError>;
}
