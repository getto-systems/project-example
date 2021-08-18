use crate::auth::{
    auth_ticket::_api::kernel::infra::{
        AuthNonceHeader, AuthTokenHeader, AuthTokenResponseBuilder,
    },
    password::reset::_common::reset::infra::ResetPasswordFieldsExtract,
};

use crate::{
    auth::{
        _api::service::data::AuthServiceError,
        auth_ticket::_common::{
            encode::data::AuthTicketEncoded,
            kernel::data::{AuthNonce, AuthToken},
        },
        password::reset::_api::reset::data::ResetPasswordMessageEncoded,
    },
    z_details::_api::message::data::MessageError,
};

pub trait ResetPasswordInfra {
    type NonceHeader: AuthNonceHeader;
    type TokenHeader: AuthTokenHeader;
    type ResetService: ResetPasswordService;
    type ResponseEncoder: ResetPasswordResponseEncoder;
    type ResponseBuilder: AuthTokenResponseBuilder;

    fn nonce_header(&self) -> &Self::NonceHeader;
    fn token_header(&self) -> &Self::TokenHeader;
    fn reset_service(&self) -> &Self::ResetService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
    fn response_builder(&self) -> &Self::ResponseBuilder;
}

pub trait ResetPasswordRequestDecoder {
    fn decode(self) -> Result<ResetPasswordFieldsExtract, MessageError>;
}

#[async_trait::async_trait]
pub trait ResetPasswordService {
    async fn reset(
        &self,
        nonce: Option<AuthNonce>,
        token: Option<AuthToken>,
        fields: ResetPasswordFieldsExtract,
    ) -> Result<ResetPasswordResponse, AuthServiceError>;
}

pub enum ResetPasswordResponse {
    Success(AuthTicketEncoded),
    InvalidReset,
    AlreadyReset,
}

pub trait ResetPasswordResponseEncoder {
    fn encode(
        &self,
        response: ResetPasswordResponse,
    ) -> Result<ResetPasswordMessageEncoded, MessageError>;
}
