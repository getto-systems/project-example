use crate::auth::auth_ticket::_api::kernel::infra::AuthHeaderInfra;

use crate::{
    auth::{
        _api::service::data::ServiceError,
        auth_ticket::_api::kernel::data::{AuthNonceValue, AuthTokenValue},
        password::reset::_api::request_token::data::RequestResetTokenResult,
    },
    z_details::_api::message::data::MessageError,
};

pub trait RequestResetTokenInfra {
    type HeaderInfra: AuthHeaderInfra;
    type RequestDecoder: RequestResetTokenRequestDecoder;
    type RequestTokenService: RequestResetTokenService;
    type ResponseEncoder: RequestResetTokenResponseEncoder;

    fn header_infra(&self) -> &Self::HeaderInfra;
    fn request_decoder(&self) -> &Self::RequestDecoder;
    fn request_token_service(&self) -> &Self::RequestTokenService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
}

pub trait RequestResetTokenRequestDecoder {
    fn decode(&self) -> Result<RequestResetTokenFieldsExtract, MessageError>;
}

#[derive(Clone)]
pub struct RequestResetTokenFieldsExtract {
    pub login_id: String,
}

#[async_trait::async_trait]
pub trait RequestResetTokenService {
    async fn request_token(
        &self,
        nonce: AuthNonceValue,
        token: AuthTokenValue,
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
