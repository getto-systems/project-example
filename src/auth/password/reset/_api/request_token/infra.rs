use crate::auth::{
    auth_ticket::_common::kernel::infra::AuthNonceMetadata,
    password::reset::_common::request_token::infra::RequestResetTokenFieldsExtract,
};

use crate::{
    auth::{
        _common::service::data::AuthServiceError, auth_ticket::_common::kernel::data::AuthNonce,
        password::reset::_api::request_token::data::RequestResetTokenResult,
    },
    z_details::_api::message::data::MessageError,
};

pub trait RequestResetTokenInfra {
    type NonceMetadata: AuthNonceMetadata;
    type RequestTokenService: RequestResetTokenService;
    type ResponseEncoder: RequestResetTokenResponseEncoder;

    fn nonce_metadata(&self) -> &Self::NonceMetadata;
    fn request_token_service(&self) -> &Self::RequestTokenService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
}

pub trait RequestResetTokenRequestDecoder {
    fn decode(self) -> Result<RequestResetTokenFieldsExtract, MessageError>;
}

#[async_trait::async_trait]
pub trait RequestResetTokenService {
    async fn request_token(
        &self,
        nonce: Option<AuthNonce>,
        fields: RequestResetTokenFieldsExtract,
    ) -> Result<RequestResetTokenResponse, AuthServiceError>;
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
