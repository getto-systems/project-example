use crate::auth::{
    auth_ticket::_common::kernel::infra::{AuthNonceMetadata, AuthTokenMetadata},
    password::_common::change::infra::ChangePasswordFieldsExtract,
};

use crate::{
    auth::{
        _common::service::data::AuthServiceError,
        auth_ticket::_common::kernel::data::{AuthNonce, AuthToken},
        password::_api::change::data::ChangePasswordResult,
    },
    z_details::_api::message::data::MessageError,
};

pub trait ChangePasswordInfra {
    type NonceMetadata: AuthNonceMetadata;
    type TokenMetadata: AuthTokenMetadata;
    type ChangeService: ChangePasswordService;
    type ResponseEncoder: ChangePasswordResponseEncoder;

    fn nonce_metadata(&self) -> &Self::NonceMetadata;
    fn token_metadata(&self) -> &Self::TokenMetadata;
    fn change_service(&self) -> &Self::ChangeService;
    fn response_encoder(&self) -> &Self::ResponseEncoder;
}

pub trait ChangePasswordRequestDecoder {
    fn decode(self) -> Result<ChangePasswordFieldsExtract, MessageError>;
}

#[async_trait::async_trait]
pub trait ChangePasswordService {
    async fn change(
        &self,
        nonce: Option<AuthNonce>,
        token: Option<AuthToken>,
        fields: ChangePasswordFieldsExtract,
    ) -> Result<ChangePasswordResponse, AuthServiceError>;
}

pub enum ChangePasswordResponse {
    Success,
    InvalidPassword,
}

pub trait ChangePasswordResponseEncoder {
    fn encode(
        &self,
        response: ChangePasswordResponse,
    ) -> Result<ChangePasswordResult, MessageError>;
}
