use crate::auth::{
    auth_ticket::_api::kernel::infra::{AuthHeaderInfra, AuthTokenInfra},
    password::_common::authenticate::infra::AuthenticatePasswordFieldsExtract,
};

use crate::{
    auth::{
        _api::service::data::ServiceError,
        auth_ticket::{
            _api::kernel::data::{AuthNonceValue, AuthTokenValue},
            _common::encode::data::EncodeAuthTicketResponse,
        },
        password::_api::authenticate::data::AuthenticatePasswordMessageEncoded,
    },
    z_details::_api::message::data::MessageError,
};

pub trait AuthenticatePasswordInfra {
    type HeaderInfra: AuthHeaderInfra;
    type TokenInfra: AuthTokenInfra;
    type RequestDecoder: AuthenticatePasswordRequestDecoder;
    type AuthenticateService: AuthenticatePasswordService;
    type ResponseEncoder: AuthenticatePasswordResponseEncoder;

    fn header_infra(&self) -> &Self::HeaderInfra;
    fn token_infra(&self) -> &Self::TokenInfra;
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
        nonce: AuthNonceValue,
        token: AuthTokenValue,
        fields: AuthenticatePasswordFieldsExtract,
    ) -> Result<AuthenticatePasswordResponse, ServiceError>;
}

pub enum AuthenticatePasswordResponse {
    Success(EncodeAuthTicketResponse),
    InvalidPassword,
}

pub trait AuthenticatePasswordResponseEncoder {
    fn encode(
        &self,
        response: AuthenticatePasswordResponse,
    ) -> Result<AuthenticatePasswordMessageEncoded, MessageError>;
}
