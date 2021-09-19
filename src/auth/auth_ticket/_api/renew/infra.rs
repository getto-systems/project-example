use crate::auth::auth_ticket::{
    _api::kernel::infra::AuthTokenResponseBuilder,
    _common::kernel::infra::{AuthServiceMetadata, AuthServiceMetadataContent},
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
    type ServiceMetadata: AuthServiceMetadata;
    type RenewService: RenewAuthTicketService;
    type ResponseEncoder: RenewAuthTicketResponseEncoder;
    type ResponseBuilder: AuthTokenResponseBuilder;

    fn service_metadata(&self) -> &Self::ServiceMetadata;
    fn renew_service(&self) -> &Self::RenewService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
    fn response_builder(&self) -> &Self::ResponseBuilder;
}

#[async_trait::async_trait]
pub trait RenewAuthTicketService {
    async fn renew(
        &self,
        metadata: AuthServiceMetadataContent,
    ) -> Result<AuthTicketEncoded, AuthServiceError>;
}

pub trait RenewAuthTicketResponseEncoder {
    fn encode(&self, response: AuthTicketEncoded) -> Result<AuthTokenMessage, MessageError>;
}
