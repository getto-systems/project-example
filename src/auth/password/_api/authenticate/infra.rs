use crate::auth::{
    auth_ticket::_api::kernel::infra::{
        AuthNonceHeader, AuthTokenHeader, AuthTokenResponseBuilder,
    },
    password::_common::authenticate::infra::AuthenticatePasswordFieldsExtract,
};

use crate::{
    auth::{
        _api::service::data::AuthServiceError,
        auth_ticket::_common::{
            encode::data::AuthTicketEncoded,
            kernel::data::{AuthNonceValue, AuthTokenValue},
        },
        password::_api::authenticate::data::AuthenticatePasswordMessageEncoded,
    },
    z_details::_api::message::data::MessageError,
};

pub trait AuthenticatePasswordInfra {
    type NonceHeader: AuthNonceHeader;
    type TokenHeader: AuthTokenHeader;
    type ResponseBuilder: AuthTokenResponseBuilder;
    type RequestDecoder: AuthenticatePasswordRequestDecoder;
    type AuthenticateService: AuthenticatePasswordService;
    type ResponseEncoder: AuthenticatePasswordResponseEncoder;

    fn nonce_header(&self) -> &Self::NonceHeader;
    fn token_header(&self) -> &Self::TokenHeader;
    fn response_builder(&self) -> &Self::ResponseBuilder;
    fn request_decoder(&self) -> &Self::RequestDecoder;
    fn authenticate_service(&self) -> &Self::AuthenticateService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
}

pub trait AuthenticatePasswordRequestDecoder {
    fn decode(&self) -> Result<AuthenticatePasswordFieldsExtract, MessageError>;
}

#[async_trait::async_trait]
pub trait AuthenticatePasswordService {
    async fn authenticate(
        &self,
        nonce: Option<AuthNonceValue>,
        token: Option<AuthTokenValue>,
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
