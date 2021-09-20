use crate::auth::{
    auth_ticket::{
        _api::kernel::infra::AuthTokenResponseBuilder,
        _common::kernel::infra::{AuthServiceMetadata, AuthServiceMetadataContent},
    },
    password::_common::authenticate::infra::AuthenticatePasswordFieldsExtract,
};

use crate::{
    auth::{
        _common::service::data::AuthServiceError,
        auth_ticket::_common::encode::data::AuthTicketEncoded,
        password::_api::authenticate::data::AuthenticatePasswordMessageEncoded,
    },
    z_details::_api::message::data::MessageError,
};

pub trait AuthenticatePasswordInfra {
    type ServiceMetadata: AuthServiceMetadata;
    type AuthenticateService: AuthenticatePasswordService;
    type ResponseEncoder: AuthenticatePasswordResponseEncoder;
    type ResponseBuilder: AuthTokenResponseBuilder;

    fn service_metadata(&self) -> &Self::ServiceMetadata;
    fn authenticate_service(&self) -> &Self::AuthenticateService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
    fn response_builder(&self) -> &Self::ResponseBuilder;
}

pub trait AuthenticatePasswordRequestDecoder {
    fn decode(self) -> Result<AuthenticatePasswordFieldsExtract, MessageError>;
}

#[async_trait::async_trait]
pub trait AuthenticatePasswordService {
    async fn authenticate(
        &self,
        metadata: AuthServiceMetadataContent,
        fields: AuthenticatePasswordFieldsExtract,
    ) -> Result<AuthenticatePasswordResponse, AuthServiceError>;
}

pub enum AuthenticatePasswordResponse {
    Success(AuthTicketEncoded),
    InvalidPassword,
}

pub trait AuthenticatePasswordResponseEncoder {
    fn encode(
        &self,
        response: AuthenticatePasswordResponse,
    ) -> Result<AuthenticatePasswordMessageEncoded, MessageError>;
}
