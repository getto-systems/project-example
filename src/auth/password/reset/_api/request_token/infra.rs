use crate::auth::{
    auth_ticket::_api::kernel::infra::{AuthNonceHeader, AuthTokenHeader},
    password::reset::_common::request_token::infra::RequestResetTokenFieldsExtract,
};

use crate::{
    auth::{
        _api::service::data::ServiceError,
        auth_ticket::_common::kernel::data::{AuthNonceValue, AuthTokenValue},
        password::reset::_api::request_token::data::RequestResetTokenResult,
    },
    z_details::_api::message::data::MessageError,
};

pub trait RequestResetTokenInfra {
    type NonceHeader: AuthNonceHeader;
    type TokenHeader: AuthTokenHeader;
    type RequestDecoder: RequestResetTokenRequestDecoder;
    type RequestTokenService: RequestResetTokenService;
    type ResponseEncoder: RequestResetTokenResponseEncoder;

    fn nonce_header(&self) -> &Self::NonceHeader;
    fn token_header(&self) -> &Self::TokenHeader;
    fn request_decoder(&self) -> &Self::RequestDecoder;
    fn request_token_service(&self) -> &Self::RequestTokenService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
}

pub trait RequestResetTokenRequestDecoder {
    fn decode(&self) -> Result<RequestResetTokenFieldsExtract, MessageError>;
}

#[async_trait::async_trait]
pub trait RequestResetTokenService {
    async fn request_token(
        &self,
        nonce: Option<AuthNonceValue>,
        token: Option<AuthTokenValue>,
        fields: RequestResetTokenFieldsExtract,
    ) -> Result<RequestResetTokenResponse, ServiceError>;
}

pub enum RequestResetTokenResponse {
    Success,
    InvalidRequest,
}

pub trait RequestResetTokenResponseEncoder {
    fn encode(
        &self,
        response: RequestResetTokenResponse,
    ) -> Result<RequestResetTokenResult, MessageError>;
}
