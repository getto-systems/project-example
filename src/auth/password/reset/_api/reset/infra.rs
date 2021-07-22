use crate::auth::auth_ticket::_api::kernel::infra::{AuthHeaderInfra, AuthTokenInfra};

use crate::{
    auth::{
        _api::service::data::ServiceError,
        auth_ticket::{
            _api::kernel::data::{AuthNonceValue, AuthTokenValue},
            _common::encode::data::EncodeAuthTicketResponse,
        },
        password::reset::_api::reset::data::ResetPasswordMessageEncoded,
    },
    z_details::_api::message::data::MessageError,
};

pub trait ResetPasswordInfra {
    type HeaderInfra: AuthHeaderInfra;
    type TokenInfra: AuthTokenInfra;
    type RequestDecoder: ResetPasswordRequestDecoder;
    type ResetService: ResetPasswordService;
    type ResponseEncoder: ResetPasswordResponseEncoder;

    fn header_infra(&self) -> &Self::HeaderInfra;
    fn token_infra(&self) -> &Self::TokenInfra;
    fn request_decoder(&self) -> &Self::RequestDecoder;
    fn reset_service(&self) -> &Self::ResetService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
}

pub trait ResetPasswordRequestDecoder {
    fn decode(&self) -> Result<ResetPasswordFieldsExtract, MessageError>;
}

// TODO common にするべき
#[derive(Clone)]
pub struct ResetPasswordFieldsExtract {
    pub login_id: String,
    pub password: String,
    pub reset_token: String,
}

#[async_trait::async_trait]
pub trait ResetPasswordService {
    async fn reset(
        &self,
        nonce: AuthNonceValue,
        token: AuthTokenValue,
        fields: ResetPasswordFieldsExtract,
    ) -> Result<ResetPasswordResponse, ServiceError>;
}

pub enum ResetPasswordResponse {
    Success(EncodeAuthTicketResponse),
    InvalidReset,
    AlreadyReset,
}

pub trait ResetPasswordResponseEncoder {
    fn encode(
        &self,
        response: ResetPasswordResponse,
    ) -> Result<ResetPasswordMessageEncoded, MessageError>;
}
